# Nostromo Notes

A retro sci-fi terminal-based note-taking application inspired by the Alien universe's Nostromo computer systems. Experience authentic 1970s-style computing with modern functionality.

## Features

### 🖥️ **Authentic Retro Interface**
- **Rozzo-style ASCII Art Header**: Beautiful "MU-TH-UR 6000" logo with perfect centering and spacing
- **Green-on-Black Terminal Aesthetic**: Authentic retro computer styling
- **Weyland-Yutani Corp Branding**: Immersive corporate sci-fi theming throughout
- **Double-line Borders**: Professional terminal UI with consistent framing
- **Real-time System Status**: Live clock, power levels, file counts, and system information

### 🗂️ **Advanced File Management** 
- **Workflow Organization**: Move notes through workflow stages:
  - 📤 **Uploaded**: Completed and uploaded notes
  - 🎨 **Rendered**: Notes ready for final review
  - 🚀 **Ready to Upload**: Processed notes awaiting upload
  - 🖨️ **Printed**: Physical copies created
- **Smart Navigation**: Arrow key navigation with up/down directory traversal
- **File Operations**: Create, edit, delete with safety confirmations
- **Directory Browsing**: Seamless folder navigation

### 🔍 **Intelligent Search System**
- **Fuzzy Search**: Real-time file filtering with partial matching
- **Blinking Cursor Effects**: Authentic terminal input experience
- **Search Results Counter**: Shows filtered vs total file counts
- **Instant Results**: Type-as-you-search functionality

### 📝 **Note Creation & Templates**
- **Template System**: Create notes from predefined templates
- **Markdown Support**: Full markdown editing with syntax support
- **Auto-save**: Automatic file saving on editor exit
- **File Preview**: Real-time content preview pane

### ⌨️ **Professional Controls**
- **Vim-inspired Navigation**: Intuitive keyboard shortcuts
- **Modal Interface**: Context-sensitive controls for different operations
- **Visual Feedback**: Clear status indicators and confirmation dialogs

## Quick Start

```bash
# Build and run
cargo run

# Essential Controls
- ↑/↓: Navigate files          - ←: Go up directory
- →/Enter: Open file/directory - /: Search files (fuzzy)
- n: Create new note           - Shift+T: New note from template  
- d: Delete file (with confirm)- m: Move to workflow stage
- c: Change root directory     - Esc: Exit current mode
- q: Quit application
```

## Development

```bash
# Development workflow
cargo check          # Quick compilation check
cargo clippy         # Linting and suggestions  
cargo fmt            # Format code
cargo run            # Build and run
cargo build --release# Optimized build
```

## Requirements

- **Rust 1.70+**: Modern Rust toolchain
- **Terminal with Unicode Support**: For proper ASCII art rendering
- **UTF-8 Locale**: For text file handling

## Architecture

- **Single Binary**: Self-contained executable
- **Configuration**: `~/.nostromo-notes.conf` for persistent settings
- **File System Integration**: Direct filesystem operations
- **Modal UI**: State-based interface with context-sensitive controls
- **Real-time Updates**: Live file system monitoring

## Project Status

✅ **Core Features Complete**:
- Retro ASCII art header with perfect spacing
- Full file management (create, edit, delete, move)
- Fuzzy search with real-time filtering
- Workflow organization system
- Template support
- Professional UI with consistent styling

🚧 **Future Enhancements**: 
- Additional ASCII art fonts
- More workflow stages
- Extended template system
- Configuration customization

---

*"MU-TH-UR 6000 mainframe online. All personnel report to designated workstations."*
*"Weyland-Yutani Corp - Building Better Worlds"*
