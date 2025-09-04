# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Project Overview

Nostromo Notes is a sophisticated terminal-based note-taking application written in Rust using the Ratatui library. It features an authentic 1970s-style retro green-on-black "Alien/Nostromo" themed interface with vim-like keybindings, advanced file management, and workflow organization capabilities. The application provides a complete note management solution with fuzzy search, template support, and immersive sci-fi theming.

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
The application follows a single-file architecture with clear separation of concerns and comprehensive feature set:

- **`App` struct**: Main application state container managing multiple modes, file operations, search state, and UI components
- **Advanced Modal System**: Nine distinct modes with specialized functionality:
  - `Normal`: File navigation and management
  - `Editing`: Full-featured text editing with auto-save
  - `Naming`: File creation with template integration
  - `ChangingDirectory`: Root directory selection
  - `SelectingTemplateFolder`: Template directory configuration
  - `SelectingTemplate`: Template file selection
  - `Search`: Real-time fuzzy search with filtering
  - `ConfirmingDelete`: Safety confirmation for destructive operations
  - `SelectingMoveDestination`: Workflow stage selection for file organization
- **Configuration**: Persistent settings in `~/.nostromo-notes.conf` with automatic management
- **Workflow Integration**: Built-in file organization system with predefined workflow stages

### Key Components

#### State Management (`App` struct)
- **Multi-modal operation**: Context-sensitive UI and input handling across nine modes
- **File system operations**: Complete CRUD operations with safety confirmations
- **Fuzzy search system**: Real-time filtering with `SkimMatcherV2` for intelligent matching
- **Workflow management**: Four-stage workflow system (Uploaded/Rendered/Ready to Upload/Printed)
- **Template system**: Full template support with directory management
- **Configuration persistence**: Automatic saving of user preferences and directory settings

#### Advanced UI System (Ratatui-based)
- **Professional Header**: Rozzo-style ASCII art "MU-TH-UR 6000" with perfect centering and spacing
- **Three-section layout**: Header (8 lines) / Main content (flexible) / Status bar (3 lines)
- **Two-pane main area**: File browser (30%) and editor/preview (70%)
- **Real-time status display**: Live clock, file counts, power indicators, mode information
- **Modal popups**: Professional confirmation dialogs and input forms
- **Consistent theming**: Double-line borders, green-on-black color scheme, Weyland-Yutani branding

#### Enhanced Input Handling
- **Arrow key navigation**: Full directional control (up/down files, left/right directories)
- **Mode-specific keybindings**: Context-aware controls that adapt to current operation
- **Visual feedback**: Blinking cursors, highlight states, progress indicators
- **Safety mechanisms**: Confirmation dialogs for destructive operations

#### Search System
- **Fuzzy matching**: Intelligent partial string matching with scoring
- **Real-time filtering**: Instant results as you type
- **Visual indicators**: Search result counts and status display
- **Seamless integration**: Works across all file operations

### Dependencies and Their Roles
- **`ratatui`**: Core TUI framework for rendering and layout management
- **`crossterm`**: Cross-platform terminal control (events, raw mode, cursor management)
- **`tui-textarea`**: Full-featured text editor widget with syntax support
- **`walkdir`**: Recursive directory traversal for file system operations
- **`dirs`**: Cross-platform user directory detection for configuration
- **`fuzzy-matcher`**: Advanced string matching with `SkimMatcherV2` for search functionality

### Configuration System
- **File location**: `~/.nostromo-notes.conf` in user home directory
- **Format**: Simple key-value pairs (`root=/path` and `template_root=/path`)
- **Auto-initialization**: Creates welcome file and default settings on first run
- **Persistent state**: Maintains user preferences across sessions
- **Directory management**: Automatic creation of workflow directories

### Complete Application Flow
1. **Normal Mode**: Primary interface for file navigation and management
2. **Search Mode**: Real-time fuzzy filtering with instant results
3. **File Operations**: Create, edit, delete, move with appropriate confirmations
4. **Workflow Management**: Organize notes through defined workflow stages
5. **Template System**: Create notes from predefined templates
6. **Directory Management**: Navigate and configure root and template directories
7. **Safety Systems**: Confirmation dialogs and operation validation

### Current Feature Set
- ✅ **Complete UI System**: Professional header, status bars, consistent theming
- ✅ **Full File Management**: CRUD operations with safety confirmations
- ✅ **Fuzzy Search**: Real-time filtering with intelligent matching
- ✅ **Workflow Organization**: Four-stage file organization system
- ✅ **Template Support**: Complete template creation and management
- ✅ **Advanced Navigation**: Arrow key navigation with directory traversal
- ✅ **Professional Styling**: Rozzo ASCII art, double borders, authentic theming
- ✅ **Real-time Status**: Live system information and progress indicators

## Key Development Considerations

### UI Architecture
- Immediate mode GUI patterns with full UI rebuild each frame
- Consistent double-line border styling across all components
- Real-time clock and status updates with second precision
- Proper text centering and spacing for professional appearance

### Error Handling Strategy
- Graceful degradation - filesystem operations use `.ok()` to prevent crashes
- User feedback through status messages and confirmation dialogs
- Fallback behaviors for missing files or directories

### Theming and Aesthetics
- Authentic 1970s retro computer styling with green-on-black color scheme
- Comprehensive Weyland-Yutani Corp branding throughout interface
- Rozzo-style ASCII art with perfect centering and balanced spacing
- Professional terminal UI conventions with consistent visual hierarchy

### File System Integration
- Hidden files excluded from listings (files starting with `.`)
- UTF-8 markdown file assumption for content handling
- Automatic directory creation for workflow stages
- Persistent configuration with automatic loading/saving
