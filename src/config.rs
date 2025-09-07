use std::{collections::HashMap, fs, path::PathBuf};
use dirs;

use crate::ui::themes::ColorScheme;
use crate::constants::DEFAULT_MOVE_DESTINATIONS;

pub fn config_path() -> Option<PathBuf> {
    dirs::home_dir().map(|mut path| {
        path.push(".nostromo-notes.conf");
        path
    })
}

pub fn load_config() -> (PathBuf, Option<PathBuf>, ColorScheme, Vec<PathBuf>) {
    let mut root = PathBuf::from(".");
    let mut template_root = None;
    let mut color_scheme = ColorScheme::Green;
    let mut workflow_folders = Vec::new();

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
            
            // Load workflow folders
            for (i, destination) in DEFAULT_MOVE_DESTINATIONS.iter().enumerate() {
                let key = format!("workflow_{}", i);
                if let Some(folder_str) = config.get(key.as_str()) {
                    workflow_folders.push(PathBuf::from(folder_str.trim()));
                } else {
                    // Default: relative to root
                    workflow_folders.push(root.join(destination));
                }
            }
        }
    }
    
    // If no workflow folders were loaded, create defaults
    if workflow_folders.is_empty() {
        for destination in DEFAULT_MOVE_DESTINATIONS {
            workflow_folders.push(root.join(destination));
        }
    }

    (root, template_root, color_scheme, workflow_folders)
}

pub fn save_config(root: &PathBuf, template_root: &Option<PathBuf>, color_scheme: &ColorScheme, workflow_folders: &Vec<PathBuf>) {
    if let Some(path) = config_path() {
        let mut content = format!("root={}\n", root.to_string_lossy());
        if let Some(tmpl_root) = template_root {
            content.push_str(&format!("template_root={}\n", tmpl_root.to_string_lossy()));
        }
        content.push_str(&format!("color_scheme={}\n", color_scheme.to_string()));
        
        // Save workflow folders
        for (i, folder) in workflow_folders.iter().enumerate() {
            content.push_str(&format!("workflow_{}={}\n", i, folder.to_string_lossy()));
        }
        
        fs::write(path, content).ok();
    }
}