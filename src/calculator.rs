use std::error::Error;
use lalrpop_util::lalrpop_mod;

// Include the generated parser code
lalrpop_mod!(pub grammar);

#[derive(PartialEq)]
pub enum CalculatorMode {
    Input,
    Result,
}

#[derive(PartialEq)]
pub enum CalculatorResult {
    Success(String),
    Error(String),
    Empty,
}

pub struct Calculator {
    pub input: String,
    pub result: CalculatorResult,
    pub mode: CalculatorMode,
}

impl Calculator {
    pub fn new() -> Calculator {
        Calculator {
            input: String::new(),
            result: CalculatorResult::Empty,
            mode: CalculatorMode::Input,
        }
    }

    pub fn evaluate(&mut self) {
        let expr = self.input.trim();
        if expr.is_empty() {
            self.result = CalculatorResult::Empty;
            return;
        }

        self.result = match self.evaluate_expression(expr) {
            Ok(val) => {
                // Format the result with appropriate rounding to handle floating point precision issues
                // Check if the value is close to an integer
                if (val - val.round()).abs() < 1e-10 {
                    CalculatorResult::Success(format!("{}", val.round() as i64))
                } else {
                    // For floating point values, format with precision and strip trailing zeros
                    let formatted = format!("{:.10}", val);
                    // Remove trailing zeros and decimal point if needed
                    let trimmed = formatted.trim_end_matches('0').trim_end_matches('.');
                    CalculatorResult::Success(trimmed.to_string())
                }
            },
            Err(m) => CalculatorResult::Error(m.to_string()),
        };
        self.mode = CalculatorMode::Result;
    }

    fn evaluate_expression(&self, expr: &str) -> Result<f64, Box<dyn Error>> {
        let parser = grammar::ExprParser::new();
        match parser.parse(expr) {
            Ok(result) => Ok(result),
            Err(_) => Err("Invalid expression".into()),
        }
    }

    pub fn handle_input(&mut self, key: crossterm::event::KeyCode) {
        match key {
            crossterm::event::KeyCode::Char(c) => {
                if self.mode == CalculatorMode::Result {
                    self.input.clear();
                    self.mode = CalculatorMode::Input;
                }
                self.input.push(c);
            }
            crossterm::event::KeyCode::Backspace => {
                if !self.input.is_empty() {
                    self.input.pop();
                }
            }
            crossterm::event::KeyCode::Enter => self.evaluate(),
            _ => {}
        }
    }
}
