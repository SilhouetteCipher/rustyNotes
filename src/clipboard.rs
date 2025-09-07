use arboard::Clipboard;
use std::process::Command;
use std::thread;
use std::time::Duration;

/// Simple markdown to clean text converter using string operations
fn simple_markdown_to_clean_text(markdown: &str) -> String {
    let mut result = String::new();
    
    for line in markdown.lines() {
        let trimmed = line.trim();
        
        if trimmed.is_empty() {
            result.push('\n');
            continue;
        }
        
        // Handle headers (convert # Header to Header with extra spacing)
        if let Some(header_text) = trimmed.strip_prefix("# ") {
            result.push_str("\n");
            result.push_str(header_text);
            result.push_str("\n\n");
            continue;
        }
        if let Some(header_text) = trimmed.strip_prefix("## ") {
            result.push_str("\n");
            result.push_str(header_text);
            result.push_str("\n\n");
            continue;
        }
        if let Some(header_text) = trimmed.strip_prefix("### ") {
            result.push_str("\n");
            result.push_str(header_text);
            result.push_str("\n\n");
            continue;
        }
        
        // Handle bullet points (convert - item to • item)
        if let Some(item_text) = trimmed.strip_prefix("- ") {
            let clean_text = clean_inline_formatting(item_text);
            result.push_str("• ");
            result.push_str(&clean_text);
            result.push('\n');
            continue;
        }
        
        // Handle numbered lists (convert 1. item to • item for simplicity)
        if trimmed.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) && trimmed.contains(". ") {
            if let Some(dot_pos) = trimmed.find(". ") {
                let item_text = &trimmed[dot_pos + 2..];
                let clean_text = clean_inline_formatting(item_text);
                result.push_str("• ");
                result.push_str(&clean_text);
                result.push('\n');
                continue;
            }
        }
        
        // Handle regular paragraphs
        let clean_text = clean_inline_formatting(trimmed);
        result.push_str(&clean_text);
        result.push('\n');
    }
    
    // Clean up excessive newlines
    result.trim().to_string()
}

/// Clean inline markdown formatting (bold, italic, code, links)
fn clean_inline_formatting(text: &str) -> String {
    let mut result = text.to_string();
    
    // Remove bold **text** and __text__
    while let Some(start) = result.find("**") {
        if let Some(end) = result[start + 2..].find("**") {
            let end_pos = start + 2 + end;
            let bold_text = &result[start + 2..start + 2 + end];
            result = format!("{}{}{}", &result[..start], bold_text, &result[end_pos + 2..]);
        } else {
            break;
        }
    }
    
    while let Some(start) = result.find("__") {
        if let Some(end) = result[start + 2..].find("__") {
            let end_pos = start + 2 + end;
            let bold_text = &result[start + 2..start + 2 + end];
            result = format!("{}{}{}", &result[..start], bold_text, &result[end_pos + 2..]);
        } else {
            break;
        }
    }
    
    // Remove italic *text* and _text_
    while let Some(start) = result.find('*') {
        if let Some(end) = result[start + 1..].find('*') {
            let end_pos = start + 1 + end;
            let italic_text = &result[start + 1..start + 1 + end];
            result = format!("{}{}{}", &result[..start], italic_text, &result[end_pos + 1..]);
        } else {
            break;
        }
    }
    
    while let Some(start) = result.find('_') {
        if let Some(end) = result[start + 1..].find('_') {
            let end_pos = start + 1 + end;
            let italic_text = &result[start + 1..start + 1 + end];
            result = format!("{}{}{}", &result[..start], italic_text, &result[end_pos + 1..]);
        } else {
            break;
        }
    }
    
    // Remove inline code `text`
    while let Some(start) = result.find('`') {
        if let Some(end) = result[start + 1..].find('`') {
            let end_pos = start + 1 + end;
            let code_text = &result[start + 1..start + 1 + end];
            result = format!("{}{}{}", &result[..start], code_text, &result[end_pos + 1..]);
        } else {
            break;
        }
    }
    
    // Remove links [text](url) - keep just the text
    while let Some(start) = result.find('[') {
        if let Some(end_bracket) = result[start..].find(']') {
            let end_bracket_pos = start + end_bracket;
            if result[end_bracket_pos + 1..].starts_with('(') {
                if let Some(end_paren) = result[end_bracket_pos + 1..].find(')') {
                    let end_paren_pos = end_bracket_pos + 1 + end_paren;
                    let link_text = &result[start + 1..end_bracket_pos];
                    result = format!("{}{}{}", &result[..start], link_text, &result[end_paren_pos + 1..]);
                } else {
                    break;
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }
    
    result
}

pub fn copy_markdown_to_clipboard(markdown_content: &str) -> Result<(), Box<dyn std::error::Error>> {
    
    let mut clipboard = Clipboard::new()?;
    
    // Convert markdown to clean text
    let clean_text = simple_markdown_to_clean_text(markdown_content);
    
    // Set clean text to system clipboard using arboard
    let _ = clipboard.set_text(clean_text.clone());
    
    // Also use wl-copy for Wayland/Hyprland compatibility
    let _ = Command::new("wl-copy")
        .arg(&clean_text)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .output();
    
    // Give clipboard managers a moment to process
    thread::sleep(Duration::from_millis(100));
    
    Ok(())
}
