use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, BorderType, Clear, List, ListItem, Paragraph};
use std::{fs, time::SystemTime};

use crate::app::App;
use crate::modes::Mode;
use crate::ui::themes::ColorScheme;
use crate::ui::components::centered_rect;
use crate::constants::WEYLAND_YUTANI_LOGO;

pub fn ui(frame: &mut Frame, app: &mut App) {
    frame.render_widget(Block::default().style(Style::default().bg(Color::Black)), frame.area());

    // Calculate current time for blinking effects and system status
    let current_time = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(16), // ASCII Header (dotted logo only)
            Constraint::Length(3),  // Corporate text pane
            Constraint::Min(0),     // Main content
            Constraint::Length(3),  // Status bar + system info
        ])
        .split(frame.area());

    render_header(frame, app, outer_layout[0]);
    render_corporate_text(frame, app, outer_layout[1]);
    
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(outer_layout[2]);

    render_left_pane(frame, app, main_layout[0], current_time);
    render_right_pane(frame, app, main_layout[1]);
    render_popups(frame, app, current_time);
    render_status_bar(frame, app, outer_layout[3], current_time);
}

fn render_header(frame: &mut Frame, app: &App, area: Rect) {
    let header_widget = Paragraph::new(WEYLAND_YUTANI_LOGO)
        .style(Style::default().fg(app.color_scheme.primary_color()).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .title(" ■■■ WEYLAND-YUTANI CORP MAINFRAME SYSTEM 6000 ■■■ ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.color_scheme.primary_color()))
                .border_type(BorderType::Double)
        );
    frame.render_widget(header_widget, area);
}

fn render_corporate_text(frame: &mut Frame, app: &App, area: Rect) {
    let corporate_text = "BUILDING BETTER WORLDS • NOSTROMO NOTES SYSTEM v2.1\n\"Our work here benefits all mankind. Our future is unlimited.\"";
    let text_widget = Paragraph::new(corporate_text)
        .style(Style::default().fg(app.color_scheme.primary_color()))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.color_scheme.primary_color()))
                .border_type(BorderType::Double)
        );
    frame.render_widget(text_widget, area);
}

fn render_left_pane(frame: &mut Frame, app: &mut App, area: Rect, current_time: u64) {
    let block_style = Style::default().fg(app.color_scheme.primary_color());

    match app.mode {
        Mode::ChangingDirectory | Mode::SelectingTemplateFolder => {
            render_browser_mode(frame, app, area, block_style);
        }
        Mode::SelectingTemplate => {
            render_template_selection(frame, app, area, block_style);
        }
        Mode::Settings => {
            render_settings_mode(frame, app, area);
        }
        _ => {
            render_file_list_mode(frame, app, area, block_style, current_time);
        }
    }
}

fn render_browser_mode(frame: &mut Frame, app: &mut App, area: Rect, block_style: Style) {
    let items: Vec<ListItem> = app.browser_entries.iter().map(|path| {
        let name = path.file_name().unwrap_or_default().to_string_lossy();
        let prefix = if path.is_dir() { "[D] " } else { "" };
        ListItem::new(format!("{}{}", prefix, name)).style(Style::default().fg(app.color_scheme.primary_color()))
    }).collect();

    let title = if app.mode == Mode::ChangingDirectory {
        format!(" Change Directory ({}) ", app.current_browser_path.to_string_lossy())
    } else {
        format!(" Select Template Directory ({}) ", app.current_browser_path.to_string_lossy())
    };

    let browser_block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(block_style)
        .border_type(BorderType::Double);

    let list = List::new(items)
        .block(browser_block)
        .highlight_style(
            Style::default()
                .bg(app.color_scheme.primary_color())
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(" > ");
    frame.render_stateful_widget(list, area, &mut app.browser_state);
}

fn render_template_selection(frame: &mut Frame, app: &mut App, area: Rect, block_style: Style) {
    let items: Vec<ListItem> = app.template_files.iter().map(|path| {
        ListItem::new(path.file_name().unwrap_or_default().to_string_lossy().to_string())
            .style(Style::default().fg(app.color_scheme.primary_color()))
    }).collect();

    let block = Block::default()
        .title(" Select Template ")
        .borders(Borders::ALL)
        .border_style(block_style)
        .border_type(BorderType::Double);

    let list = List::new(items)
        .block(block)
        .highlight_style(
            Style::default()
                .bg(app.color_scheme.primary_color())
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(" > ");
    frame.render_stateful_widget(list, area, &mut app.template_list_state);
}

fn render_settings_mode(frame: &mut Frame, app: &mut App, area: Rect) {
    let schemes = ColorScheme::all_schemes();
    let items: Vec<ListItem> = schemes
        .iter()
        .map(|scheme| {
            let name = scheme.name();
            let indicator = if *scheme == app.color_scheme { " ● " } else { " ○ " };
            let preview = "█████ SAMPLE TEXT █████";
            let item_text = format!("{}{:<12} {}", indicator, name, preview);
            ListItem::new(item_text)
                .style(Style::default().fg(scheme.primary_color()))
        })
        .collect();

    let settings_block = Block::default()
        .title(" ■■■ WEYLAND-YUTANI COLOR SCHEME SETTINGS ■■■ ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.color_scheme.primary_color()))
        .border_type(BorderType::Double);

    let list = List::new(items)
        .block(settings_block)
        .highlight_style(
            Style::default()
                .bg(app.color_scheme.primary_color())
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("► ");
    frame.render_stateful_widget(list, area, &mut app.settings_selection_state);
}

fn render_file_list_mode(frame: &mut Frame, app: &mut App, area: Rect, block_style: Style, current_time: u64) {
    let left_pane_area = if app.mode == Mode::Search {
        let search_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(area);
        
        // Render search bar with enhanced cursor effect
        let (cursor_char, search_title, search_style) = if app.search_input_mode {
            // Input mode: fast blinking cursor, highlighted background
            let cursor = if (current_time % 2) == 0 { "█" } else { " " };
            (cursor, " ■■■ SEARCH ARCHIVE [TYPING] ■■■ ", Style::default().fg(app.color_scheme.primary_color()).bg(Color::Black))
        } else {
            // Navigation mode: no cursor, different title, dimmed style
            ("", " ■■■ SEARCH RESULTS [LOCKED] ■■■ ", Style::default().fg(app.color_scheme.secondary_color()).bg(Color::Black))
        };
        let search_display = format!("{}{}", app.search_input, cursor_char);
        let search_widget = Paragraph::new(search_display)
            .style(search_style)
            .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(app.color_scheme.primary_color())).title(search_title).border_type(BorderType::Double));
        frame.render_widget(search_widget, search_layout[0]);
        
        search_layout[1]
    } else {
        area
    };

    let title = if app.mode == Mode::Search {
        format!(" ■■■ SEARCH RESULTS ({}/{}) ■■■ ", app.filtered_files.len(), app.files.len())
    } else {
        format!(" ■■■ MU-TH-UR 6000 FILE ARCHIVE ({}) ■■■ ", app.root.to_string_lossy())
    };

    let file_list_block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(block_style)
        .border_type(BorderType::Double);

    let current_files = app.get_current_files();
    let items: Vec<ListItem> = current_files
        .iter()
        .enumerate()
        .map(|(i, path)| {
            let filename = path.file_name().unwrap_or_default().to_string_lossy();
            let (prefix, style) = if path.is_dir() { 
                ("▶ [DIR]", Style::default().fg(app.color_scheme.secondary_color()).add_modifier(Modifier::BOLD)) 
            } else { 
                ("■ [FILE]", Style::default().fg(app.color_scheme.primary_color())) 
            };
            ListItem::new(format!("{:02} {} {}", i + 1, prefix, filename)).style(style)
        })
        .collect();

    let file_list = List::new(items)
        .block(file_list_block)
        .highlight_style(
            Style::default()
                .bg(app.color_scheme.primary_color())
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("► ");

    frame.render_stateful_widget(file_list, left_pane_area, &mut app.file_list_state);
}

fn render_right_pane(frame: &mut Frame, app: &mut App, area: Rect) {
    let block_style = Style::default().fg(app.color_scheme.primary_color());

    if app.mode == Mode::Settings {
        render_settings_preview(frame, app, area);
    } else if let Some(editor) = app.editor.as_mut() {
        frame.render_widget(&*editor, area);
    } else {
        render_file_preview(frame, app, area, block_style);
    }
}

fn render_settings_preview(frame: &mut Frame, app: &App, area: Rect) {
    let schemes = ColorScheme::all_schemes();
    let selected_scheme = schemes.get(app.settings_selection_state.selected().unwrap_or(0))
        .unwrap_or(&ColorScheme::Green);
    let info_content = format!(
        "\n\n\n████████████████████████████████\n█       COLOR SCHEME PREVIEW       █\n████████████████████████████████\n\n\nSelected: {}\n\nPrimary Color: {:#?}\nSecondary Color: {:#?}\n\nThis theme will be applied to:\n• Interface borders and highlights\n• Text and status information\n• File browser and editor\n\nPress ENTER to apply this theme\nPress ESC to cancel",
        selected_scheme.name(),
        selected_scheme.primary_color(),
        selected_scheme.secondary_color()
    );
    
    let info_block = Block::default()
        .title(" ■■■ THEME PREVIEW ■■■ ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(selected_scheme.primary_color()))
        .border_type(BorderType::Double);

    let info_paragraph = Paragraph::new(info_content)
        .style(Style::default().fg(selected_scheme.primary_color()))
        .block(info_block);

    frame.render_widget(info_paragraph, area);
}

fn render_file_preview(frame: &mut Frame, app: &App, area: Rect, block_style: Style) {
    let (title, content) = if let Some(selected_index) = app.file_list_state.selected() {
        let current_files = app.get_current_files();
        current_files.get(selected_index)
            .and_then(|path| {
                if path.is_dir() {
                    Some(("[DIR]".to_string(), path.to_string_lossy().into_owned()))
                } else {
                    fs::read_to_string(path).ok().map(|content| (path.file_name().unwrap_or_default().to_string_lossy().into_owned(), content))
                }
            })
            .map(|(name, content)| (format!(" ■■■ VIEWING: {} ■■■ ", name), content))
            .unwrap_or_else(|| (" ■■■ WEYLAND-YUTANI CORP - FILE VIEWER ■■■ ".to_string(), "\n\n\n\n████████████████████\n█     < PREVIEW NOT AVAILABLE >     █\n████████████████████".to_string()))
    } else {
        (" ■■■ WEYLAND-YUTANI CORP - FILE VIEWER ■■■ ".to_string(), "\n\n\n████████████████████████████████\n█  < SELECT A FILE FROM THE MU-TH-UR >  █\n█  < 6000 MAINFRAME ARCHIVE SYSTEM >  █\n████████████████████████████████".to_string())
    };

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(block_style)
        .border_type(BorderType::Double);

    let paragraph = Paragraph::new(content)
        .style(Style::default().fg(app.color_scheme.primary_color()))
        .block(block);

    frame.render_widget(paragraph, area);
}

fn render_popups(frame: &mut Frame, app: &mut App, current_time: u64) {
    match app.mode {
        Mode::Naming => render_naming_popup(frame, app, current_time),
        Mode::Renaming => render_rename_popup(frame, app, current_time),
        Mode::ConfirmingDelete => render_delete_confirmation_popup(frame, app),
        Mode::SelectingMoveDestination => render_move_destination_popup(frame, app),
        _ => {}
    }
}

fn render_naming_popup(frame: &mut Frame, app: &App, current_time: u64) {
    let area = centered_rect(70, 3, frame.area());
    let title = if app.pending_template.is_some() {
        " ■■■ CREATE NEW NOTE FROM TEMPLATE ■■■ "
    } else {
        " ■■■ CREATE NEW NOTE FILE ■■■ "
    };
    let cursor_char = if (current_time % 2) == 0 { "█" } else { " " };
    let input_display = format!("{}{}", app.filename_input, cursor_char);
    let input_widget = Paragraph::new(input_display)
        .style(Style::default().fg(app.color_scheme.primary_color()).bg(Color::Black))
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(app.color_scheme.primary_color())).title(title).border_type(BorderType::Double));
    frame.render_widget(Clear, area);
    frame.render_widget(input_widget, area);
}

fn render_rename_popup(frame: &mut Frame, app: &App, current_time: u64) {
    let area = centered_rect(70, 3, frame.area());
    let title = " ■■■ RENAME FILE ■■■ ";
    let cursor_char = if (current_time % 2) == 0 { "█" } else { " " };
    let input_display = format!("{}{}", app.filename_input, cursor_char);
    let input_widget = Paragraph::new(input_display)
        .style(Style::default().fg(app.color_scheme.primary_color()).bg(Color::Black))
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(app.color_scheme.primary_color())).title(title).border_type(BorderType::Double));
    frame.render_widget(Clear, area);
    frame.render_widget(input_widget, area);
}

fn render_delete_confirmation_popup(frame: &mut Frame, app: &App) {
    let area = centered_rect(60, 9, frame.area());
    let filename = app.operation_target_file
        .as_ref()
        .and_then(|p| p.file_name())
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "Unknown".to_string());
    
    // Clean, properly aligned content without conflicting borders
    let content = format!(
        "\n\n    ⚠  WARNING: DESTRUCTIVE OPERATION  ⚠\n\n\n    DELETE FILE: '{}'?\n\n\n    [Y] CONFIRM     [N/ESC] CANCEL\n\n",
        filename
    );
    
    let delete_widget = Paragraph::new(content)
        .style(Style::default().fg(Color::Red).bg(Color::Black).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" ■■■ WEYLAND-YUTANI SECURITY PROTOCOL ■■■ ")
                .border_style(Style::default().fg(Color::Red))
                .border_type(BorderType::Double)
        );
    
    frame.render_widget(Clear, area);
    frame.render_widget(delete_widget, area);
}

fn render_move_destination_popup(frame: &mut Frame, app: &mut App) {
    let area = centered_rect(50, 8, frame.area());
    let filename = app.operation_target_file
        .as_ref()
        .and_then(|p| p.file_name())
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "Unknown".to_string());
    
    let items: Vec<ListItem> = app.move_destinations
        .iter()
        .map(|dest| ListItem::new(dest.as_str()).style(Style::default().fg(app.color_scheme.primary_color())))
        .collect();
    
    let block_style = Style::default().fg(app.color_scheme.primary_color());
    let list = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(format!(" ■■■ RELOCATE FILE: '{}' TO WORKFLOW STAGE ■■■ ", filename))
            .border_style(block_style)
            .border_type(BorderType::Double))
        .highlight_style(
            Style::default()
                .bg(app.color_scheme.primary_color())
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("► ");
    
    frame.render_widget(Clear, area);
    frame.render_stateful_widget(list, area, &mut app.move_selection_state);
}

fn render_status_bar(frame: &mut Frame, app: &App, area: Rect, current_time: u64) {
    let status_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Length(1), Constraint::Length(1)])
        .split(area);

    // Enhanced System Status Line with retro metrics and blinking status lights
    let hours = (current_time / 3600) % 24;
    let minutes = (current_time / 60) % 60;
    let seconds = current_time % 60;
    let uptime = current_time - app.app_start_time;
    
    // Generate fake retro computer metrics
    let cpu_load = ((current_time * 7) % 100) + 15; // Pseudo-random 15-115%
    let memory_used = 8 + ((current_time * 3) % 56); // 8-64K range
    let temperature = 18 + ((current_time * 2) % 15); // 18-33°C range
    
    // Static status lights - no blinking to avoid stuttering
    let system_info = format!(
        "███ MU-TH-UR 6000 ███ {:02}:{:02}:{:02} ███ UPTIME: {}S ███ CPU: {}% ███ MEM: {}K/64K ███ TEMP: {}°C ███ PWR:● SYS:● LIFE:●",
        hours, minutes, seconds, uptime, cpu_load, memory_used, temperature
    );
    let system_bar = Paragraph::new(system_info)
        .style(Style::default().fg(app.color_scheme.primary_color()).bg(Color::Black).add_modifier(Modifier::BOLD));
    frame.render_widget(system_bar, status_layout[0]);

    // Mode and Path Status Line  
    let mode_info = format!(
        "███ MODE: {} ███ DIRECTORY: {} ███",
        app.mode.to_string(),
        app.root.to_string_lossy()
    );
    let mode_bar = Paragraph::new(mode_info)
        .style(Style::default().fg(Color::Black).bg(app.color_scheme.primary_color()));
    frame.render_widget(mode_bar, status_layout[1]);

    // Controls Line
    let controls_text = match app.mode {
        Mode::Normal => "▶ NAV: ↑/↓/←/→ ▶ NEW: n ▶ RENAME: r ▶ SEARCH: / ▶ TMPL: Shift+T ▶ CHDIR: c ▶ DEL: d ▶ MOVE: m ▶ SETTINGS: s ▶ QUIT: q",
        Mode::Editing => "▶ SAVE & EXIT: Esc ▶ COPY: Ctrl+C",
        Mode::Naming => "▶ CONFIRM: Enter ▶ CANCEL: Esc",
        Mode::Renaming => "▶ CONFIRM: Enter ▶ CANCEL: Esc",
        Mode::ChangingDirectory => "▶ SELECT: s ▶ NAVIGATE: ↑/↓/Enter ▶ CANCEL: Esc",
        Mode::SelectingTemplateFolder => "▶ SELECT: s ▶ NAVIGATE: ↑/↓/Enter ▶ CANCEL: Esc",
        Mode::SelectingTemplate => "▶ SELECT: Enter ▶ CANCEL: Esc",
        Mode::Search => if app.search_input_mode {
            "▶ TYPE QUERY ▶ LOCK INPUT: Enter ▶ CANCEL SEARCH: Esc"
        } else {
            "▶ NAV: ↑/↓/←/→ ▶ EDIT QUERY: / ▶ DEL: d ▶ MOVE: m ▶ RENAME: r ▶ OPEN: Enter/→ ▶ EXIT: Esc"
        },
        Mode::ConfirmingDelete => "▶ CONFIRM: Y/Enter ▶ CANCEL: N/Esc",
        Mode::SelectingMoveDestination => "▶ SELECT: Enter ▶ NAVIGATE: ↑/↓ ▶ CANCEL: Esc",
        Mode::Settings => "▶ APPLY: Enter ▶ NAVIGATE: ↑/↓ ▶ CANCEL: Esc",
    };
    
    let controls_bar = Paragraph::new(controls_text)
        .style(Style::default().fg(app.color_scheme.primary_color()).bg(Color::Black));
    frame.render_widget(controls_bar, status_layout[2]);
}