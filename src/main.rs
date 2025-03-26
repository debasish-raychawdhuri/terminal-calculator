use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{error::Error, io, time::{Duration, Instant}};

mod calculator;
mod ui;

use calculator::Calculator;

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>, Box<dyn Error>> {
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend).map_err(|e| e.into())
}

fn cleanup_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut terminal = setup_terminal()?;
    let mut calculator = Calculator::new();
    let mut should_quit = false;
    
    // Animation frame rate (refresh every 100ms)
    let frame_rate = Duration::from_millis(100);
    let mut last_tick = Instant::now();

    while !should_quit {
        // Draw the UI
        ui::draw_ui(&mut terminal, &calculator.input, &calculator.result)?;
        
        // Calculate time until next frame
        let timeout = frame_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or(Duration::from_millis(0));

        // Poll for events with timeout to ensure animation continues
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match (key.code, key.modifiers) {
                    (KeyCode::Char('c'), KeyModifiers::CONTROL) | (KeyCode::Esc, _) => {
                        should_quit = true;
                    }
                    (key_code, _) => calculator.handle_input(key_code),
                }
            }
        }
        
        // Check if it's time for a new frame
        if last_tick.elapsed() >= frame_rate {
            last_tick = Instant::now();
        }
    }

    cleanup_terminal(&mut terminal)?;
    Ok(())
}
