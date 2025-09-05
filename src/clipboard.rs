use arboard::Clipboard;
use pulldown_cmark::{html, Options, Parser};
use std::process::Command;
use std::thread;
use std::time::Duration;

pub fn copy_markdown_to_clipboard(markdown_content: &str) -> Result<(), Box<dyn std::error::Error>> {
    
    let mut clipboard = Clipboard::new()?;
    
    // Set text to system clipboard using arboard
    let _ = clipboard.set_text(markdown_content.to_string());
    
    // Also use wl-copy for Wayland/Hyprland compatibility
    let _ = Command::new("wl-copy")
        .arg(markdown_content)
        .output();
    
    // Also try wl-copy with stdin (more reliable on some systems)
    if let Ok(mut child) = Command::new("wl-copy")
        .stdin(std::process::Stdio::piped())
        .spawn() 
    {
        if let Some(mut stdin) = child.stdin.take() {
            use std::io::Write;
            let _ = stdin.write_all(markdown_content.as_bytes());
            let _ = stdin.flush();
        }
        let _ = child.wait();
    }
    
    // Convert markdown to HTML for rich text editors
    let options = Options::empty();
    let parser = Parser::new_ext(markdown_content, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    
    // Try to set HTML format as well (for rich text compatibility)
    let _ = clipboard.set_html(&html_output, Some(&markdown_content.to_string()));
    
    // Give clipboard managers a moment to process
    thread::sleep(Duration::from_millis(100));
    
    Ok(())
}
