# CRUSH.md - Nostromo Notes Development Guide

## Build/Test/Lint Commands
```bash
# Build and run the application
cargo run

# Development workflow
cargo check                    # Quick compilation check
cargo clippy                   # Linting and suggestions
cargo fmt                      # Format code
cargo build                    # Debug build
cargo build --release          # Optimized release build

# No unit tests currently - this is a single-binary TUI application
```

## Code Style Guidelines

### Imports
- Group imports: std library first, then external crates, then local modules
- Use explicit imports for commonly used items (e.g., `use ratatui::prelude::*`)
- Prefer qualified paths for disambiguation when needed

### Formatting & Structure
- Use `cargo fmt` for consistent formatting (Rust standard style)
- 4-space indentation, no tabs
- Line length: aim for 100 characters, break at logical points
- Use trailing commas in multi-line structures

### Types & Naming
- Use `PascalCase` for types, enums, structs (e.g., `ColorScheme`, `Mode`)
- Use `snake_case` for functions, variables, modules (e.g., `load_files`, `file_list_state`)
- Use `SCREAMING_SNAKE_CASE` for constants
- Prefer descriptive names over abbreviations

### Error Handling
- Use `Result<T>` type alias: `type Result<T> = std::result::Result<T, Box<dyn Error>>`
- Use `?` operator for error propagation
- Use `.ok()` for operations where failure is acceptable (e.g., config file operations)
- Avoid `unwrap()` except in cases where panic is intended

### Architecture Patterns
- Single main struct (`App`) holds all application state
- Use enums for mode management (`Mode` enum with explicit states)
- Implement methods on main struct for state transitions
- Use `match` expressions for handling different modes/states
- Keep UI rendering separate from business logic