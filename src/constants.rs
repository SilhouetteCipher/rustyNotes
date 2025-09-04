pub const DEFAULT_MOVE_DESTINATIONS: &[&str] = &[
    "Uploaded",
    "Rendered", 
    "Ready to Upload",
    "Printed",
];

pub const WELCOME_FILE_CONTENT: &str = "# Welcome to Nostromo Notes

This is a retro-themed notes editor.

Controls:
- Up/Down arrows: Navigate files
- Left arrow: Go up one directory level
- Right arrow/Enter: Open file or enter directory
- n: Create new note
- /: Search files (fuzzy filter)
- Shift+T: New note from template
- c: Change directory
- d: Delete file (with confirmation)
- m: Move file to workflow stage (Uploaded/Rendered/Ready to Upload/Printed)
- Esc: Save and exit editor / Exit search
- q: Quit application";

pub const ROZZO_ASCII_ART: &str = r#"
   e   e     8888 8888     88P'888'Y88 888 888     8888 8888 888 88e      e88",8,    e88 88e     e88 88e     e88 88e   
  d8b d8b    8888 8888     P'  888  'Y 888 888     8888 8888 888 888D    d888  "    d888 888b   d888 888b   d888 888b  
 e Y8b Y8b   8888 8888 888     888     8888888 888 8888 8888 888 88"    C8888 88e  C8888 8888D C8888 8888D C8888 8888D 
d8b Y8b Y8b  8888 8888         888     888 888     8888 8888 888 b,      Y888 888D  Y888 888P   Y888 888P   Y888 888P  
888b Y8b Y8b 'Y88 88P'         888     888 888     'Y88 88P' 888 88b,     "88 88"    "88 88"     "88 88"     "88 88"   
"#;