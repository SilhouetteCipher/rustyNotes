use std::{fs, path::PathBuf};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, BorderType};
use tui_textarea::TextArea;
use fuzzy_matcher::FuzzyMatcher;

use crate::app::App;
use crate::modes::Mode;
use crate::ui::themes::ColorScheme;
use crate::file_ops::{load_files, load_browser_entries, load_template_files};

impl<'a> App<'a> {
    pub fn load_files(&mut self) {
        self.files = load_files(&self.root);
    }

    pub fn load_browser_entries(&mut self) {
        self.browser_entries = load_browser_entries(&self.current_browser_path);
        self.browser_state.select(Some(0));
    }

    pub fn enter_directory_browser(&mut self, for_templates: bool) {
        self.current_browser_path = self.root.clone();
        self.load_browser_entries();
        self.mode = if for_templates { Mode::SelectingTemplateFolder } else { Mode::ChangingDirectory };
    }

    pub fn select_browser_entry(&mut self) {
        if let Some(index) = self.browser_state.selected() {
            if let Some(path) = self.browser_entries.get(index) {
                if path.is_dir() {
                    self.current_browser_path = fs::canonicalize(path).unwrap_or_else(|_| path.clone());
                    self.load_browser_entries();
                }
            }
        }
    }

    pub fn set_new_root(&mut self) {
        self.root = self.current_browser_path.clone();
        self.save_config();
        self.load_files();
        self.file_list_state.select(Some(0));
        self.mode = Mode::Normal;
    }

    pub fn set_template_root(&mut self) {
        self.template_root = Some(self.current_browser_path.clone());
        self.save_config();
        self.start_template_workflow();
    }

    pub fn start_template_workflow(&mut self) {
        if let Some(template_root) = &self.template_root {
            self.template_files = load_template_files(template_root);
            self.template_list_state.select(Some(0));
            self.mode = Mode::SelectingTemplate;
        } else {
            self.enter_directory_browser(true);
        }
    }

    pub fn select_template(&mut self) {
        if let Some(index) = self.template_list_state.selected() {
            if let Some(path) = self.template_files.get(index).cloned() {
                self.pending_template = Some(path);
                self.mode = Mode::Naming;
            }
        }
    }

    pub fn create_new_note(&mut self) {
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
                fs::write(&new_path, "").ok();
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

    pub fn select_next(&mut self) {
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

    pub fn select_previous(&mut self) {
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

    pub fn start_editing(&mut self) {
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
                    self.editing_file_path = Some(path.clone());
                    self.mode = Mode::Editing;
                } else {
                    self.root = path;
                    self.load_files();
                    self.file_list_state.select(Some(0));
                    if self.mode == Mode::Search {
                        self.exit_search_mode();
                    }
                }
            }
        }
    }

    pub fn stop_editing(&mut self) {
        if let (Some(editor), Some(path)) = (self.editor.take(), self.editing_file_path.take()) {
            fs::write(path, editor.lines().join("\n")).ok();
        }
        self.mode = Mode::Normal;
    }

    pub fn enter_search_mode(&mut self) {
        self.mode = Mode::Search;
        self.search_input.clear();
        self.update_filtered_files();
    }

    pub fn exit_search_mode(&mut self) {
        self.mode = Mode::Normal;
        self.search_input.clear();
        self.filtered_files.clear();
        if !self.files.is_empty() {
            self.file_list_state.select(Some(0));
        }
    }

    pub fn update_filtered_files(&mut self) {
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
            
            scored_files.sort_by(|a, b| b.1.cmp(&a.1));
            self.filtered_files = scored_files.into_iter().map(|(path, _)| path).collect();
        }
        
        if !self.filtered_files.is_empty() {
            self.file_list_state.select(Some(0));
        } else {
            self.file_list_state.select(None);
        }
    }

    pub fn navigate_up_directory(&mut self) {
        if let Some(parent) = self.root.parent() {
            self.root = parent.to_path_buf();
            self.save_config();
            self.load_files();
            self.file_list_state.select(Some(0));
            if self.mode == Mode::Search {
                self.exit_search_mode();
            }
        }
    }

    pub fn start_delete_confirmation(&mut self) {
        if let Some(selected_index) = self.file_list_state.selected() {
            let current_files = self.get_current_files();
            if let Some(path) = current_files.get(selected_index).cloned() {
                if path.is_file() {
                    self.operation_target_file = Some(path);
                    self.mode = Mode::ConfirmingDelete;
                }
            }
        }
    }

    pub fn confirm_delete(&mut self) {
        if let Some(path) = self.operation_target_file.take() {
            fs::remove_file(path).ok();
            self.load_files();
            if !self.files.is_empty() {
                let new_selection = self.file_list_state.selected().unwrap_or(0).min(self.files.len() - 1);
                self.file_list_state.select(Some(new_selection));
            } else {
                self.file_list_state.select(None);
            }
            if self.mode == Mode::ConfirmingDelete {
                self.mode = Mode::Normal;
            }
            if !self.search_input.is_empty() {
                self.update_filtered_files();
            }
        }
        self.mode = Mode::Normal;
    }

    pub fn cancel_operation(&mut self) {
        self.operation_target_file = None;
        self.mode = Mode::Normal;
    }

    pub fn start_move_selection(&mut self) {
        if let Some(selected_index) = self.file_list_state.selected() {
            let current_files = self.get_current_files();
            if let Some(path) = current_files.get(selected_index).cloned() {
                if path.is_file() {
                    self.operation_target_file = Some(path);
                    self.move_selection_state.select(Some(0));
                    self.mode = Mode::SelectingMoveDestination;
                }
            }
        }
    }

    pub fn move_selection_next(&mut self) {
        let count = self.move_destinations.len();
        let i = match self.move_selection_state.selected() {
            Some(i) => {
                if i >= count - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.move_selection_state.select(Some(i));
    }

    pub fn move_selection_previous(&mut self) {
        let count = self.move_destinations.len();
        let i = match self.move_selection_state.selected() {
            Some(i) => {
                if i == 0 {
                    count - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.move_selection_state.select(Some(i));
    }

    pub fn execute_move(&mut self) {
        if let (Some(path), Some(dest_index)) = (self.operation_target_file.take(), self.move_selection_state.selected()) {
            if let Some(dest_folder) = self.move_destinations.get(dest_index) {
                let dest_path = self.root.join(dest_folder);
                if let Some(filename) = path.file_name() {
                    let new_path = dest_path.join(filename);
                    fs::create_dir_all(&dest_path).ok();
                    fs::rename(path, new_path).ok();
                    self.load_files();
                    if !self.files.is_empty() {
                        let new_selection = self.file_list_state.selected().unwrap_or(0).min(self.files.len() - 1);
                        self.file_list_state.select(Some(new_selection));
                    } else {
                        self.file_list_state.select(None);
                    }
                    if !self.search_input.is_empty() {
                        self.update_filtered_files();
                    }
                }
            }
        }
        self.mode = Mode::Normal;
    }

    pub fn enter_settings(&mut self) {
        self.settings_selection_state.select(Some(0));
        self.mode = Mode::Settings;
    }

    pub fn exit_settings(&mut self) {
        self.mode = Mode::Normal;
    }

    pub fn settings_next(&mut self) {
        let schemes = ColorScheme::all_schemes();
        let i = match self.settings_selection_state.selected() {
            Some(i) => {
                if i >= schemes.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.settings_selection_state.select(Some(i));
    }

    pub fn settings_previous(&mut self) {
        let schemes = ColorScheme::all_schemes();
        let i = match self.settings_selection_state.selected() {
            Some(i) => {
                if i == 0 {
                    schemes.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.settings_selection_state.select(Some(i));
    }

    pub fn apply_color_scheme(&mut self) {
        if let Some(index) = self.settings_selection_state.selected() {
            let schemes = ColorScheme::all_schemes();
            if let Some(scheme) = schemes.get(index) {
                self.color_scheme = *scheme;
                self.save_config();
            }
        }
    }
}