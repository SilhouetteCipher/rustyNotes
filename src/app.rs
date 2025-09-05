use ratatui::widgets::{ListState};
use std::{path::PathBuf, time::{SystemTime, UNIX_EPOCH}};
use tui_textarea::TextArea;
use fuzzy_matcher::skim::SkimMatcherV2;

use crate::modes::Mode;
use crate::ui::themes::ColorScheme;
use crate::constants::DEFAULT_MOVE_DESTINATIONS;
use crate::config;

pub struct App<'a> {
    pub mode: Mode,
    pub root: PathBuf,
    pub template_root: Option<PathBuf>,
    pub files: Vec<PathBuf>,
    pub file_list_state: ListState,
    pub editor: Option<TextArea<'a>>,
    pub filename_input: String,
    pub pending_template: Option<PathBuf>,
    pub browser_entries: Vec<PathBuf>,
    pub browser_state: ListState,
    pub current_browser_path: PathBuf,
    pub template_files: Vec<PathBuf>,
    pub template_list_state: ListState,
    pub search_input: String,
    pub search_input_mode: bool, // true = typing search, false = navigating results
    pub filtered_files: Vec<PathBuf>,
    pub fuzzy_matcher: SkimMatcherV2,
    pub editing_file_path: Option<PathBuf>,
    pub operation_target_file: Option<PathBuf>,
    pub move_destinations: Vec<String>,
    pub move_selection_state: ListState,
    pub color_scheme: ColorScheme,
    pub settings_selection_state: ListState,
    // Animation timing fields
    pub app_start_time: u64,
    pub last_update_time: u64,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        let (root, template_root, color_scheme) = config::load_config();
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        let mut app = Self {
            mode: Mode::Normal,
            root,
            template_root,
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
            search_input_mode: true,
            filtered_files: Vec::new(),
            fuzzy_matcher: SkimMatcherV2::default(),
            editing_file_path: None,
            operation_target_file: None,
            move_destinations: DEFAULT_MOVE_DESTINATIONS.iter().map(|s| s.to_string()).collect(),
            move_selection_state: ListState::default(),
            color_scheme,
            settings_selection_state: ListState::default(),
            app_start_time: current_time,
            last_update_time: current_time,
        };
        
        app.load_files();
        if !app.files.is_empty() {
            app.file_list_state.select(Some(0));
        }
        app
    }

    pub fn save_config(&self) {
        config::save_config(&self.root, &self.template_root, &self.color_scheme);
    }

    pub fn get_current_files(&self) -> &Vec<PathBuf> {
        if self.mode == Mode::Search {
            &self.filtered_files
        } else {
            &self.files
        }
    }
}