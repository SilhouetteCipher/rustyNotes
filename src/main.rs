use ratatui::crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, BorderType, Clear, List, ListItem, ListState, Paragraph};
use std::{collections::HashMap, error::Error, fs, io, path::PathBuf};
use tui_textarea::TextArea;
use walkdir::WalkDir;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn config_path() -> Option<PathBuf> {
    dirs::home_dir().map(|mut path| {
        path.push(".nostromo-notes.conf");
        path
    })
}

#[derive(PartialEq, Clone, Copy)]
enum Mode {
    Normal,
    Editing,
    Naming,
    ChangingDirectory,
    SelectingTemplateFolder,
    SelectingTemplate,
    Search,
}

impl Mode {
    fn to_string(&self) -> &str {
        match self {
            Mode::Normal => "NAVIGATE",
            Mode::Editing => "EDITING",
            Mode::Naming => "NAMING",
            Mode::ChangingDirectory => "CHANGE DIR",
            Mode::SelectingTemplateFolder => "SELECT TMPL DIR",
            Mode::SelectingTemplate => "SELECT TMPL",
            Mode::Search => "SEARCH",
        }
    }
}

struct App<'a> {
    mode: Mode,
    root: PathBuf,
    template_root: Option<PathBuf>,
    files: Vec<PathBuf>,
    file_list_state: ListState,
    editor: Option<TextArea<'a>>,
    filename_input: String,
    pending_template: Option<PathBuf>,
    browser_entries: Vec<PathBuf>,
    browser_state: ListState,
    current_browser_path: PathBuf,
    template_files: Vec<PathBuf>,
    template_list_state: ListState,
    search_input: String,
    filtered_files: Vec<PathBuf>,
    fuzzy_matcher: SkimMatcherV2,
}

impl<'a> App<'a> {
    fn new() -> Self {
        let mut app = Self {
            mode: Mode::Normal,
            root: PathBuf::from("."),
            template_root: None,
            files: Vec::new(),
            file_list_state: ListState::default(),
            editor: None,
            filename_input: String::new(),
            pending_template: None,
            browser_entries: Vec::new(),
            browser_state: ListState::default(),
            current_browser_path: PathBuf::from("."),
            template_files: Vec::new(),
            template_list_state: ListState::default(),
            search_input: String::new(),
            filtered_files: Vec::new(),
            fuzzy_matcher: SkimMatcherV2::default(),
        };
        app.load_config();
        app.load_files();
        if !app.files.is_empty() {
            app.file_list_state.select(Some(0));
        }
        app
    }

    fn load_config(&mut self) {
        if let Some(path) = config_path() {
            if let Ok(content) = fs::read_to_string(path) {
                let config: HashMap<_, _> = content
                    .lines()
                    .filter_map(|line| line.split_once('='))
                    .collect();
                if let Some(root_str) = config.get("root") {
                    self.root = PathBuf::from(root_str.trim());
                }
                if let Some(tmpl_str) = config.get("template_root") {
                    let path = PathBuf::from(tmpl_str.trim());
                    if path.is_dir() {
                        self.template_root = Some(path);
                    }
                }
            }
        }
    }

    fn save_config(&self) {
        if let Some(path) = config_path() {
            let mut content = format!("root={}\n", self.root.to_string_lossy());
            if let Some(tmpl_root) = &self.template_root {
                content.push_str(&format!("template_root={}\n", tmpl_root.to_string_lossy()));
            }
            fs::write(path, content).ok();
        }
    }

    fn load_files(&mut self) {
        let walkdir = WalkDir::new(&self.root).max_depth(1);
        let mut entries: Vec<PathBuf> = walkdir
            .into_iter()
            .filter_map(|e| e.ok())
            .map(|e| e.path().to_path_buf())
            .filter(|p| {
                *p != self.root &&
                !p.file_name().unwrap_or_default().to_string_lossy().starts_with(".")
            })
            .collect();

        entries.sort_by(|a, b| {
            let a_is_dir = a.is_dir();
            let b_is_dir = b.is_dir();
            b_is_dir.cmp(&a_is_dir).then_with(|| a.cmp(b))
        });

        self.files = entries;
    }

    fn load_browser_entries(&mut self) {
        self.browser_entries.clear();
        if self.current_browser_path.parent().is_some() {
            self.browser_entries.push(self.current_browser_path.join(".."));
        }

        if let Ok(entries) = fs::read_dir(&self.current_browser_path) {
            let mut dirs: Vec<PathBuf> = entries
                .filter_map(|res| res.ok())
                .map(|e| e.path())
                .filter(|path| {
                    path.is_dir() && 
                    !path.file_name().unwrap_or_default().to_string_lossy().starts_with(".")
                })
                .collect();
            dirs.sort();
            self.browser_entries.append(&mut dirs);
        }
        self.browser_state.select(Some(0));
    }

    fn enter_directory_browser(&mut self, for_templates: bool) {
        self.current_browser_path = self.root.clone();
        self.load_browser_entries();
        self.mode = if for_templates { Mode::SelectingTemplateFolder } else { Mode::ChangingDirectory };
    }

    fn select_browser_entry(&mut self) {
        if let Some(index) = self.browser_state.selected() {
            if let Some(path) = self.browser_entries.get(index) {
                if path.is_dir() {
                    self.current_browser_path = fs::canonicalize(path).unwrap_or_else(|_| path.clone());
                    self.load_browser_entries();
                }
            }
        }
    }

    fn set_new_root(&mut self) {
        self.root = self.current_browser_path.clone();
        self.save_config();
        self.load_files();
        self.file_list_state.select(Some(0));
        self.mode = Mode::Normal;
    }

    fn set_template_root(&mut self) {
        self.template_root = Some(self.current_browser_path.clone());
        self.save_config();
        self.start_template_workflow(); // Continue to selecting a template
    }

    fn start_template_workflow(&mut self) {
        if let Some(template_root) = &self.template_root {
            self.template_files = WalkDir::new(template_root)
                .max_depth(1)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file())
                .map(|e| e.into_path())
                .collect();
            self.template_list_state.select(Some(0));
            self.mode = Mode::SelectingTemplate;
        } else {
            self.enter_directory_browser(true);
        }
    }

    fn select_template(&mut self) {
        if let Some(index) = self.template_list_state.selected() {
            if let Some(path) = self.template_files.get(index).cloned() {
                self.pending_template = Some(path);
                self.mode = Mode::Naming;
            }
        }
    }

    fn create_new_note(&mut self) {
        let filename = self.filename_input.clone();
        if filename.is_empty() {
            self.mode = Mode::Normal;
            return;
        }

        let filename_with_ext = if filename.ends_with(".md") {
            filename
        } else {
            format!("{}.md", filename)
        };

        let new_path = self.root.join(filename_with_ext);
        
        if let Some(template_path) = self.pending_template.take() {
            if let Ok(content) = fs::read_to_string(template_path) {
                fs::write(&new_path, content).ok();
            } else {
                fs::write(&new_path, "").ok(); // Fallback to empty file
            }
        } else {
            fs::write(&new_path, "").ok();
        }

        self.load_files();
        let new_file_index = self.files.iter().position(|f| f == &new_path);
        if let Some(index) = new_file_index {
            self.file_list_state.select(Some(index));
            self.start_editing();
        } else {
            self.mode = Mode::Normal;
        }
        self.filename_input.clear();
    }

    fn select_next(&mut self) {
        let (state, count) = match self.mode {
            Mode::ChangingDirectory | Mode::SelectingTemplateFolder => (&mut self.browser_state, self.browser_entries.len()),
            Mode::SelectingTemplate => (&mut self.template_list_state, self.template_files.len()),
            Mode::Search => (&mut self.file_list_state, self.filtered_files.len()),
            _ => (&mut self.file_list_state, self.files.len()),
        };

        let i = match state.selected() {
            Some(i) => {
                if count == 0 || i >= count - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        state.select(Some(i));
    }

    fn select_previous(&mut self) {
        let (state, count) = match self.mode {
            Mode::ChangingDirectory | Mode::SelectingTemplateFolder => (&mut self.browser_state, self.browser_entries.len()),
            Mode::SelectingTemplate => (&mut self.template_list_state, self.template_files.len()),
            Mode::Search => (&mut self.file_list_state, self.filtered_files.len()),
            _ => (&mut self.file_list_state, self.files.len()),
        };

        let i = match state.selected() {
            Some(i) => {
                if i == 0 {
                    count.saturating_sub(1)
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        state.select(Some(i));
    }

    fn start_editing(&mut self) {
        if let Some(selected_index) = self.file_list_state.selected() {
            let current_files = self.get_current_files();
            if let Some(path) = current_files.get(selected_index).cloned() {
                if path.is_file() {
                    let content = fs::read_to_string(&path).unwrap_or_default();
                    let lines: Vec<String> = content.lines().map(String::from).collect();
                    let mut editor = TextArea::new(lines);
                    let block_style = Style::default().fg(Color::Green);
                    editor.set_block(
                        Block::default()
                            .title(" Editor (Press Esc to Save) ")
                            .borders(Borders::ALL)
                            .border_style(block_style)
                            .border_type(BorderType::Double),
                    );
                    editor.set_style(Style::default().fg(Color::Green).bg(Color::Black));
                    self.editor = Some(editor);
                    self.mode = Mode::Editing;
                } else {
                    // This is a directory, so we should enter it
                    self.root = path;
                    self.load_files();
                    self.file_list_state.select(Some(0));
                    // Exit search mode when entering directory
                    if self.mode == Mode::Search {
                        self.exit_search_mode();
                    }
                }
            }
        }
    }

    fn stop_editing(&mut self) {
        if let Some(selected_index) = self.file_list_state.selected() {
            let current_files = if self.mode == Mode::Editing {
                // When editing, we need to use the files from before we entered edit mode
                // For simplicity, we'll store the path in the editor and find it in the main files list
                &self.files
            } else {
                self.get_current_files()
            };
            if let (Some(editor), Some(path)) = (self.editor.take(), current_files.get(selected_index)) {
                fs::write(path, editor.lines().join("\n")).ok();
            }
        }
        self.mode = Mode::Normal;
    }

    fn enter_search_mode(&mut self) {
        self.mode = Mode::Search;
        self.search_input.clear();
        self.update_filtered_files();
    }

    fn exit_search_mode(&mut self) {
        self.mode = Mode::Normal;
        self.search_input.clear();
        self.filtered_files.clear();
        // Reset selection to the first item
        if !self.files.is_empty() {
            self.file_list_state.select(Some(0));
        }
    }

    fn update_filtered_files(&mut self) {
        if self.search_input.is_empty() {
            self.filtered_files = self.files.clone();
        } else {
            let mut scored_files: Vec<(PathBuf, i64)> = self.files
                .iter()
                .filter_map(|path| {
                    let filename = path.file_name()?.to_string_lossy();
                    self.fuzzy_matcher
                        .fuzzy_match(&filename, &self.search_input)
                        .map(|score| (path.clone(), score))
                })
                .collect();
            
            // Sort by score (higher is better)
            scored_files.sort_by(|a, b| b.1.cmp(&a.1));
            self.filtered_files = scored_files.into_iter().map(|(path, _)| path).collect();
        }
        
        // Reset selection to the first item in filtered results
        if !self.filtered_files.is_empty() {
            self.file_list_state.select(Some(0));
        } else {
            self.file_list_state.select(None);
        }
    }

    fn get_current_files(&self) -> &Vec<PathBuf> {
        if self.mode == Mode::Search {
            &self.filtered_files
        } else {
            &self.files
        }
    }
}

fn main() -> Result<()> {
    if fs::metadata("welcome.md").is_err() {
        fs::write("welcome.md", "# Welcome to Nostromo Notes\n\nThis is a retro-themed notes editor.\n\nControls:\n- Up/Down: Navigate files\n- Enter: Edit file or enter directory\n- n: Create new note\n- d: Change directory\n- Shift+T: New note from template\n- Esc: Save and exit editor\n- q: Quit application")?;
    }

    let mut terminal = setup_terminal()?;
    let mut app = App::new();
    run(&mut terminal, &mut app)?;
    restore_terminal(&mut terminal)?;
    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

fn run<'a>(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App<'a>,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            let current_mode = app.mode;
            match current_mode {
                Mode::Normal => match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('n') => app.mode = Mode::Naming,
                    KeyCode::Char('d') => app.enter_directory_browser(false),
                    KeyCode::Char('T') => app.start_template_workflow(),
                    KeyCode::Char('/') => app.enter_search_mode(),
                    KeyCode::Down => app.select_next(),
                    KeyCode::Up => app.select_previous(),
                    KeyCode::Enter => app.start_editing(),
                    _ => {} // Ignore other keys
                },
                Mode::Editing => match key.code {
                    KeyCode::Esc => app.stop_editing(),
                    _ => {
                        if let Some(editor) = app.editor.as_mut() {
                            editor.input(key);
                        }
                    }
                },
                Mode::Naming => match key.code {
                    KeyCode::Enter => app.create_new_note(),
                    KeyCode::Esc => {
                        app.filename_input.clear();
                        app.pending_template = None;
                        app.mode = Mode::Normal;
                    }
                    KeyCode::Char(c) => {
                        app.filename_input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.filename_input.pop();
                    }
                    _ => {} // Ignore other keys
                },
                Mode::ChangingDirectory => match key.code {
                    KeyCode::Esc => app.mode = Mode::Normal,
                    KeyCode::Char('s') => app.set_new_root(),
                    KeyCode::Enter => app.select_browser_entry(),
                    KeyCode::Down => app.select_next(),
                    KeyCode::Up => app.select_previous(),
                    _ => {} // Ignore other keys
                },
                Mode::SelectingTemplateFolder => match key.code {
                    KeyCode::Esc => app.mode = Mode::Normal,
                    KeyCode::Char('s') => app.set_template_root(),
                    KeyCode::Enter => app.select_browser_entry(),
                    KeyCode::Down => app.select_next(),
                    KeyCode::Up => app.select_previous(),
                    _ => {} // Ignore other keys
                },
                Mode::SelectingTemplate => match key.code {
                    KeyCode::Esc => app.mode = Mode::Normal,
                    KeyCode::Enter => app.select_template(),
                    KeyCode::Down => app.select_next(),
                    KeyCode::Up => app.select_previous(),
                    _ => {} // Ignore other keys
                },
            }
        }
    }
}

fn ui(frame: &mut Frame, app: &mut App) {
    frame.render_widget(Block::default().style(Style::default().bg(Color::Black)), frame.area());

    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(frame.area());

    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(outer_layout[0]);

    let block_style = Style::default().fg(Color::Green);

    // --- Left Pane ---
    match app.mode {
        Mode::ChangingDirectory | Mode::SelectingTemplateFolder => {
            let items: Vec<ListItem> = app.browser_entries.iter().map(|path| {
                let name = path.file_name().unwrap_or_default().to_string_lossy();
                let prefix = if path.is_dir() { "[D] " } else { "" };
                ListItem::new(format!("{}{}", prefix, name)).style(Style::default().fg(Color::Green))
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
                        .bg(Color::Green)
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(" > ");
            frame.render_stateful_widget(list, main_layout[0], &mut app.browser_state);
        }
        Mode::SelectingTemplate => {
            let items: Vec<ListItem> = app.template_files.iter().map(|path| {
                ListItem::new(path.file_name().unwrap_or_default().to_string_lossy().to_string())
                    .style(Style::default().fg(Color::Green))
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
                        .bg(Color::Green)
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(" > ");
            frame.render_stateful_widget(list, main_layout[0], &mut app.template_list_state);
        }
        _ => { // Normal mode
            let file_list_block = Block::default() 
                .title(format!(" MU-TH-UR 6000 ({}) ", app.root.to_string_lossy()))
                .borders(Borders::ALL)
                .border_style(block_style)
                .border_type(BorderType::Double);

            let items: Vec<ListItem> = app
                .files
                .iter()
                .map(|path| {
                    let filename = path.file_name().unwrap_or_default().to_string_lossy();
                    let prefix = if path.is_dir() { "[D] " } else { "" };
                    ListItem::new(format!("{}{}", prefix, filename)).style(Style::default().fg(Color::Green))
                })
                .collect();

            let file_list = List::new(items)
                .block(file_list_block)
                .highlight_style(
                    Style::default()
                        .bg(Color::Green)
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(" > ");

            frame.render_stateful_widget(file_list, main_layout[0], &mut app.file_list_state);
        }
    }

    // --- Editor/Preview Pane ---
    if let Some(editor) = app.editor.as_mut() {
        frame.render_widget(&*editor, main_layout[1]);
    } else {
        let (title, content) = if let Some(selected_index) = app.file_list_state.selected() {
            app.files.get(selected_index)
                .and_then(|path| {
                    if path.is_dir() {
                        Some(("[DIR]".to_string(), path.to_string_lossy().into_owned()))
                    } else {
                        fs::read_to_string(path).ok().map(|content| (path.file_name().unwrap_or_default().to_string_lossy().into_owned(), content))
                    }
                })
                .map(|(name, content)| (format!(" Preview: {} ", name), content))
                .unwrap_or_else(|| ("Weyland-Yutani Corp - File Viewer ".to_string(), "\n\n\n\n\n        < PREVIEW NOT AVAILABLE >".to_string()))
        } else {
            ("Weyland-Yutani Corp - File Viewer ".to_string(), "\n\n\n\n\n        < SELECT A FILE FROM THE MU-TH-UR 6000 ARCHIVE >".to_string())
        };

        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(block_style)
            .border_type(BorderType::Double);

        let paragraph = Paragraph::new(content)
            .style(Style::default().fg(Color::Green))
            .block(block);

        frame.render_widget(paragraph, main_layout[1]);
    }

    // --- Naming Popup ---
    if app.mode == Mode::Naming {
        let area = centered_rect(60, 3, frame.area());
        let title = if app.pending_template.is_some() {
            " New Note From Template "
        } else {
            " New Note Name "
        };
        let input_widget = Paragraph::new(app.filename_input.as_str())
            .style(Style::default().fg(Color::Green).bg(Color::Black))
            .block(Block::default().borders(Borders::ALL).title(title));
        frame.render_widget(Clear, area);
        frame.render_widget(input_widget, area);
    }

    // --- Status Bar ---
    let controls_text = match app.mode {
        Mode::Normal => "NEW: n | TMPL: Shift+T | CHDIR: d | QUIT: q",
        Mode::Editing => "SAVE & EXIT: Esc",
        Mode::Naming => "CONFIRM: Enter | CANCEL: Esc",
        Mode::ChangingDirectory => "SELECT: s | NAVIGATE: ↑/↓/Enter | CANCEL: Esc",
        Mode::SelectingTemplateFolder => "SELECT: s | NAVIGATE: ↑/↓/Enter | CANCEL: Esc",
        Mode::SelectingTemplate => "SELECT: Enter | CANCEL: Esc",
    };

    let status_text = format!(
        " MODE: {} | ROOT: {} | {}",
        app.mode.to_string(),
        app.root.to_string_lossy(),
        controls_text
    );
    let status_bar = Paragraph::new(status_text)
        .style(Style::default().fg(Color::Black).bg(Color::Green));
    frame.render_widget(status_bar, outer_layout[1]);
}

/// Helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, height: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - height) / 2),
            Constraint::Length(height),
            Constraint::Percentage((100 - height) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
