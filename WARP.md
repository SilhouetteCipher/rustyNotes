# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Project Overview

Nostromo Notes is a terminal-based note-taking application written in Rust using the Ratatui library. It features a retro green-on-black "Alien/Nostromo" themed interface with vim-like keybindings for navigation and file management.

## Common Commands

### Building and Running
```bash
# Check for compilation errors (recommended before committing)
cargo check

# Build the application
cargo build

# Run the application
cargo run

# Build optimized release version
cargo build --release
```

### Testing and Linting
```bash
# Run tests (note: currently no tests are implemented)
cargo test

# Run Clippy for linting
cargo clippy

# Format code
cargo fmt
```

### Development Workflow
```bash
# Quick development cycle: check, fix, run
cargo check && cargo run

# Fix formatting issues
cargo fmt

# Full quality check before commit
cargo clippy && cargo fmt && cargo check
```

## Architecture Overview

### Application Structure
The application follows a single-file architecture with clear separation of concerns:

- **`App` struct**: Main application state container managing multiple modes, file lists, editor state, and configuration
- **Modal System**: Six distinct modes (`Normal`, `Editing`, `Naming`, `ChangingDirectory`, `SelectingTemplateFolder`, `SelectingTemplate`) with mode-specific UI and input handling
- **Configuration**: Persistent settings stored in `~/.nostromo-notes.conf` with automatic loading/saving
- **File System Integration**: Built-in directory browser and file management with template support

### Key Components

#### State Management (`App` struct)
- **Multi-modal operation**: Each mode has distinct UI rendering and input handling
- **File system operations**: Directory navigation, file listing, and content management
- **Template system**: Support for creating notes from template files
- **Configuration persistence**: Root directory and template directory settings

#### UI System (Ratatui-based)
- **Two-pane layout**: File browser (left) and editor/preview (right)
- **Stateful widgets**: List navigation with highlight states for files, directories, and templates
- **Modal popups**: Input dialogs for naming files and selecting options
- **Real-time preview**: File content preview when not in editing mode

#### Input Handling
- **Mode-specific keybindings**: Different key behaviors depending on current application mode
- **Editor integration**: Full text editing capabilities via `tui-textarea` crate
- **Directory browsing**: Navigation through filesystem with visual indicators

### Dependencies and Their Roles
- **`ratatui`**: Core TUI framework for rendering and layout
- **`crossterm`**: Cross-platform terminal control (events, raw mode, cursor)
- **`tui-textarea`**: Full-featured text editor widget
- **`walkdir`**: Recursive directory traversal for file discovery
- **`dirs`**: Cross-platform user directory detection

### Configuration System
- Configuration file: `~/.nostromo-notes.conf`
- Simple key-value format: `root=/path/to/notes` and `template_root=/path/to/templates`
- Automatic creation of welcome file on first run
- Persistent settings across sessions

### Application Modes Flow
1. **Normal**: File navigation and selection
2. **Editing**: Full text editing with automatic save on exit
3. **Naming**: File creation with optional template selection
4. **Directory browsing**: Changing root directory or selecting template directories
5. **Template selection**: Choosing from available template files

## Key Development Considerations

- The application uses immediate mode GUI patterns where UI is rebuilt each frame
- Error handling is minimal - most filesystem operations use `.ok()` to ignore errors
- The retro theme is hardcoded with green-on-black color scheme and "Weyland-Yutani Corp" branding
- Directory listings exclude hidden files (starting with `.`)
- All text files are assumed to be UTF-8 encoded markdown files
