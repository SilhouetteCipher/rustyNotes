use std::{collections::HashMap, fs, path::PathBuf};
use dirs;

use crate::ui::themes::ColorScheme;

pub fn config_path() -> Option<PathBuf> {
    dirs::home_dir().map(|mut path| {
        path.push(".nostromo-notes.conf");
        path
    })
}

pub fn load_config() -> (PathBuf, Option<PathBuf>, ColorScheme) {
    let mut root = PathBuf::from(".");
    let mut template_root = None;
    let mut color_scheme = ColorScheme::Green;

    if let Some(path) = config_path() {
        if let Ok(content) = fs::read_to_string(path) {
            let config: HashMap<_, _> = content
                .lines()
                .filter_map(|line| line.split_once('='))
                .collect();
            
            if let Some(root_str) = config.get("root") {
                root = PathBuf::from(root_str.trim());
            }
            
            if let Some(tmpl_str) = config.get("template_root") {
                let path = PathBuf::from(tmpl_str.trim());
                if path.is_dir() {
                    template_root = Some(path);
                }
            }
            
            if let Some(color_str) = config.get("color_scheme") {
                color_scheme = ColorScheme::from_string(color_str.trim());
            }
        }
    }

    (root, template_root, color_scheme)
}

pub fn save_config(root: &PathBuf, template_root: &Option<PathBuf>, color_scheme: &ColorScheme) {
    if let Some(path) = config_path() {
        let mut content = format!("root={}\n", root.to_string_lossy());
        if let Some(tmpl_root) = template_root {
            content.push_str(&format!("template_root={}\n", tmpl_root.to_string_lossy()));
        }
        content.push_str(&format!("color_scheme={}\n", color_scheme.to_string()));
        fs::write(path, content).ok();
    }
}