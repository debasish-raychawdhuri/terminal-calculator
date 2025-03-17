# Terminal Calculator

A beautiful terminal-based calculator written in Rust, featuring a modern TUI (Text User Interface), syntax highlighting, and animated visuals.

![Terminal Calculator Demo](demo.gif)

## Features

- 🧮 **Full Arithmetic Support**
  - Basic operations: `+`, `-`, `*`, `/`
  - Proper operator precedence
  - Parentheses for grouping expressions

- 🎨 **Beautiful Interface**
  - Color-coded input with syntax highlighting
  - Animated "C" logo
  - Clear result display with success/error states
  - Helpful command reference

- 🌈 **Visual Feedback**
  - Green for successful calculations
  - Red for error messages
  - Color-coded matching parentheses
  - Yellow highlights for important information

## Quick Start

### Prerequisites

- Rust (1.75 or later)
- Cargo (included with Rust)

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/terminal_calculator.git
   cd terminal_calculator
   ```

2. Build and run:
   ```bash
   cargo run
   ```

## Usage

The calculator provides an intuitive interface with the following controls:

- **Enter**: Calculate the result
- **Esc/Ctrl+C**: Quit the application

### Example Expressions

- Simple: `5 + 3`
- Complex: `(2 + 3) * (4 - 1)`
- Nested: `(10 + (5 * 2)) / 4`

## Technical Details

Built with modern Rust libraries:
- **ratatui**: Terminal User Interface framework
- **crossterm**: Terminal manipulation and event handling
- **LALRPOP**: Parser generator for expression parsing

## Project Structure

```
terminal_calculator/
├── src/
│   ├── main.rs       # Entry point and event handling
│   ├── ui.rs         # TUI components and rendering
│   ├── calculator.rs # Core calculation logic
│   ├── grammar.lalrpop # Expression parser grammar
│   └── lib.rs        # Library interface
├── Cargo.toml        # Dependencies and metadata
└── README.md        # This file
```

## Contributing

Contributions are welcome! Feel free to:
1. Fork the repository
2. Create a feature branch
3. Submit a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with ❤️ using [Rust](https://www.rust-lang.org/)
- TUI powered by [ratatui](https://github.com/tui-rs-revival/ratatui)
- Parser generated with [LALRPOP](https://github.com/lalrpop/lalrpop)
