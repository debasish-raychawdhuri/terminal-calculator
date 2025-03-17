use std::error::Error;
use std::io;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::{time::{Duration, Instant}, sync::Mutex};

use crate::calculator::CalculatorResult;

const ANIMATION_FRAMES: [&str; 8] = [
    "       \n   \n  \n  \n   \n       ",
    "       cccc\n   \n  \n  \n   \n       ",
    "       cccc\n   ccc \n  \n  \n   \n       ",
    "       cccc\n   ccc \ncc \n  \n   \n       ",
    "       cccc\n   ccc \ncc \ncc \n   \n       ",
    "       cccc\n   ccc \ncc \ncc \n   ccc \n       ",
    "       cccc\n   ccc \ncc \ncc \n   ccc \n       cccc",
    "       cccc\n   ccc \ncc \ncc \n   ccc \n       cccc"
];

struct AnimationState {
    frame: usize,
    last_update: Instant,
    frame_duration: Duration,
}

impl AnimationState {
    fn new() -> Self {
        Self {
            frame: 0,
            last_update: Instant::now(),
            frame_duration: Duration::from_millis(100),
        }
    }

    fn update(&mut self) -> String {
        let now = Instant::now();
        if now.duration_since(self.last_update) >= self.frame_duration {
            self.frame = (self.frame + 1) % ANIMATION_FRAMES.len();
            self.last_update = now;
        }
        ANIMATION_FRAMES[self.frame].to_string()
    }
}

static ANIMATION_STATE: Mutex<Option<AnimationState>> = Mutex::new(None);

pub fn draw_ui(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    input: &str,
    result: &CalculatorResult,
) -> Result<(), Box<dyn Error>> {
    // Animation state is initialized lazily in the draw closure
    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default()
            .title(Span::styled(
                "Calculator",
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
            ))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan));

        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(2)
            .constraints([
                Constraint::Min(40),  // Main content
                Constraint::Length(15), // Animation area
            ])
            .split(size);

        let left_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(1),
            ])
            .split(main_chunks[0]);

        let input_spans = colorize_input(input);
        let mut spans = vec![Span::styled(
            "Input: ",
            Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD),
        )];
        spans.extend(input_spans);
        
        let input = Paragraph::new(Line::from(spans))
            .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Blue)));

        let result_text = match result {
            CalculatorResult::Success(val) => (
                Color::Green,
                format!("Result: {}", val)
            ),
            CalculatorResult::Error(err) => (
                Color::Red,
                format!("Error: {}", err)
            ),
            CalculatorResult::Empty => (
                Color::DarkGray,
                String::from("Result: ")
            ),
        };

        let result = Paragraph::new(Line::from(vec![Span::styled(
            result_text.1,
            Style::default()
                .fg(result_text.0)
                .add_modifier(Modifier::BOLD),
        )]))
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(result_text.0)));

        let mut animation_guard = ANIMATION_STATE.lock().unwrap();
        if animation_guard.is_none() {
            *animation_guard = Some(AnimationState::new());
        }
        let animation_frame = animation_guard.as_mut().unwrap().update();

        let help = Paragraph::new(vec![
            Line::from(vec![Span::styled("Enter", Style::default().fg(Color::Green)), Span::raw(": Calculate")]),
            Line::from(vec![Span::styled("Esc/Ctrl+C", Style::default().fg(Color::Red)), Span::raw(": Quit")]),
            Line::from(vec![Span::styled("Examples:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))]),
            Line::from(vec![Span::raw("  Simple: "), Span::styled("5 + 3", Style::default().fg(Color::White))]),
            Line::from(colorize_example("  Complex: (2 + 3) * (4 - 1)")),
            Line::from(colorize_example("  Nested: (10 + (5 * 2)) / 4")),
            Line::from(vec![Span::styled("Operators:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)), Span::styled(" +, -, *, /", Style::default().fg(Color::Green))]),
            Line::from(vec![Span::styled("Features:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))]),
            Line::from(vec![Span::raw("  - "), Span::styled("Full arithmetic expressions", Style::default().fg(Color::Cyan))]),
            Line::from(vec![Span::raw("  - "), Span::styled("Parentheses for grouping", Style::default().fg(Color::Cyan))]),
            Line::from(vec![Span::raw("  - "), Span::styled("Operator precedence", Style::default().fg(Color::Cyan))]),
        ])
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow))
            .title(Span::styled("Help", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))));

        f.render_widget(block, size);
        f.render_widget(input, left_chunks[0]);
        f.render_widget(result, left_chunks[1]);
        f.render_widget(help, left_chunks[2]);

        // Render the animated C
        let animation_text = animation_frame.lines()
            .map(|line| Line::from(vec![Span::styled(line, Style::default().fg(Color::Yellow))]))
            .collect::<Vec<_>>();

        let animation = Paragraph::new(animation_text)
            .alignment(ratatui::layout::Alignment::Center);

        f.render_widget(animation, main_chunks[1]);
    })?;
    Ok(())
}

fn get_paren_color(level: usize) -> Color {
    match level % 5 {
        0 => Color::Yellow,
        1 => Color::Magenta,
        2 => Color::Cyan,
        3 => Color::Green,
        4 => Color::Red,
        _ => unreachable!(),
    }
}

fn colorize_input(input: &str) -> Vec<Span> {
    let mut spans = Vec::new();
    let mut paren_stack = Vec::new();
    let mut current_start = 0;
    
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => {
                // Add text before parenthesis
                if current_start < i {
                    spans.push(Span::styled(
                        &input[current_start..i],
                        Style::default().fg(Color::White)
                    ));
                }
                
                let level = paren_stack.len();
                paren_stack.push(i);
                spans.push(Span::styled(
                    "(",
                    Style::default().fg(get_paren_color(level))
                ));
                current_start = i + 1;
            },
            ')' => {
                if let Some(_) = paren_stack.pop() {
                    // Add text before closing parenthesis
                    if current_start < i {
                        spans.push(Span::styled(
                            &input[current_start..i],
                            Style::default().fg(Color::White)
                        ));
                    }
                    
                    let level = paren_stack.len();
                    spans.push(Span::styled(
                        ")",
                        Style::default().fg(get_paren_color(level))
                    ));
                    current_start = i + 1;
                }
            },
            _ => {}
        }
    }
    
    // Add remaining text
    if current_start < input.len() {
        spans.push(Span::styled(
            &input[current_start..],
            Style::default().fg(Color::White)
        ));
    }
    
    spans
}

fn colorize_example(text: &str) -> Vec<Span> {
    let parts: Vec<&str> = text.split(":").collect();
    if parts.len() != 2 {
        return vec![Span::raw(text)];
    }
    
    let mut spans = vec![Span::raw(parts[0]), Span::raw(":")];
    spans.extend(colorize_input(parts[1].trim()));
    spans
}


