# Nostromo Notes

A retro sci-fi terminal-based note-taking application inspired by the Alien universe's Nostromo computer systems. Experience authentic 1970s-style computing with modern functionality.

## Features

### 🖥️ **Authentic Retro Interface**
- **Rozzo-style ASCII Art Header**: Beautiful "MU-TH-UR 6000" logo with perfect centering and spacing
- **Multiple Color Themes**: Seven classic terminal color schemes with live preview
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

### 🔍 **Advanced Search System**
- **Two-Phase Fuzzy Search**: Type query, then navigate results separately
- **Intelligent Cursor**: Blinking cursor in input mode, clean navigation mode
- **Search Results Counter**: Shows filtered vs total file counts
- **Instant Results**: Real-time filtering as you type
- **Visual Mode Indicators**: Clear [TYPING] vs [LOCKED] status display

### 📝 **Note Creation & Templates**
- **Template System**: Create notes from predefined templates
- **Markdown Support**: Full markdown editing with syntax support
- **Auto-save**: Automatic file saving on editor exit
- **File Preview**: Real-time content preview pane

### 🎨 **Customizable Themes**
- **Seven Color Schemes**: Choose from classic terminal colors:
  - 🟢 **Classic Green**: Traditional terminal green
  - 🔵 **Terminal Blue**: Blue/cyan retro theme
  - 🟡 **Retro Amber**: Yellow amber theme
  - 🟠 **Bright Orange**: Vibrant orange theme
  - 🟢 **Light Green**: Lighter green variant
  - 🔴 **Alert Red**: Standard red theme
  - 🔴 **Vibrant Red**: Intense bright red theme
- **Live Preview**: See themes instantly before applying
- **Persistent Settings**: Theme preferences saved automatically

### ⌨️ **Professional Controls**
- **Vim-inspired Navigation**: Intuitive keyboard shortcuts
- **Modal Interface**: Context-sensitive controls for different operations  
- **Visual Feedback**: Clear status indicators and confirmation dialogs
- **Clipboard Integration**: Copy file content as HTML/plain text for CMS compatibility
- **File Renaming**: In-place file renaming with visual confirmation
- **Retro System Status**: Live CPU, memory, temperature metrics with status lights

## Installation & Usage

### **Quick Setup (New Machine)**
```bash
# Clone the repository
git clone <repository-url>
cd nostromo-notes

# Install system-wide (recommended)
cargo install --path .

# Add cargo bin to PATH (if not already added)
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# Launch from anywhere
nostromo-notes
```

### **Alternative: Development Build**
```bash
# Development build (slower)
cargo run

# Release build (faster, optimized)
cargo build --release
./target/release/nostromo-notes
```

### **Essential Controls**
```bash
# Navigation
- ↑/↓: Navigate files          - ←: Go up directory
- →/Enter: Open file/directory - /: Search files (fuzzy)

# File Operations  
- n: Create new note           - r: Rename file
- Shift+T: New note from template
- d: Delete file (with confirm)- m: Move to workflow stage
- Ctrl+C: Copy file content (in editor)

# System
- c: Change root directory     - s: Settings (color themes)
- Esc: Exit current mode       - q: Quit application
```

### **Advanced Search**
```bash
# Two-Phase Search System:
1. Press "/" to enter search mode
2. Type your query (fuzzy matching)
3. Press "Enter" to lock search and navigate results
4. Press "/" again to modify search query
5. Press "Esc" to exit search
```

## Development

```bash
# Development workflow
cargo check          # Quick compilation check
cargo clippy         # Linting and suggestions  
cargo fmt            # Format code
cargo run            # Build and run (debug)
cargo build --release# Optimized production build
cargo install --path .# Install system-wide
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
- 🖥️ **Authentic Retro Interface**: MU-TH-UR 6000 ASCII art with Weyland-Yutani branding
- 🗋 **Advanced File Management**: Create, edit, delete, rename, move with confirmations
- 🔍 **Two-Phase Fuzzy Search**: Input mode + navigation mode with visual indicators  
- 📁 **Workflow Organization**: Four-stage workflow system for note management
- 📄 **Template System**: Create notes from predefined templates
- 🎨 **Seven Color Themes**: Live preview with instant theme switching
- ⚙️ **Settings Interface**: Dedicated settings screen for customization
- 📋 **Clipboard Integration**: Copy content as HTML + plain text for CMS compatibility
- 🖥️ **System Monitoring**: Live CPU, memory, temperature with status indicators
- 🎯 **Professional UI**: Consistent double-line borders and retro styling

🚧 **Future Enhancements**: 
- Additional ASCII art fonts
- More workflow stages
- Extended template system
- Additional customization options

---

*"MU-TH-UR 6000 mainframe online. All personnel report to designated workstations."*
*"Weyland-Yutani Corp - Building Better Worlds"*
