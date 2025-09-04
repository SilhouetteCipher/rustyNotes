use std::{fs, path::PathBuf};
use walkdir::WalkDir;

pub fn load_files(root: &PathBuf) -> Vec<PathBuf> {
    let walkdir = WalkDir::new(root).max_depth(1);
    let mut entries: Vec<PathBuf> = walkdir
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_path_buf())
        .filter(|p| {
            *p != *root &&
            !p.file_name().unwrap_or_default().to_string_lossy().starts_with(".")
        })
        .collect();

    entries.sort_by(|a, b| {
        let a_is_dir = a.is_dir();
        let b_is_dir = b.is_dir();
        b_is_dir.cmp(&a_is_dir).then_with(|| a.cmp(b))
    });

    entries
}

pub fn load_browser_entries(current_path: &PathBuf) -> Vec<PathBuf> {
    let mut entries = Vec::new();
    
    if current_path.parent().is_some() {
        entries.push(current_path.join(".."));
    }

    if let Ok(dir_entries) = fs::read_dir(current_path) {
        let mut dirs: Vec<PathBuf> = dir_entries
            .filter_map(|res| res.ok())
            .map(|e| e.path())
            .filter(|path| {
                path.is_dir() && 
                !path.file_name().unwrap_or_default().to_string_lossy().starts_with(".")
            })
            .collect();
        dirs.sort();
        entries.append(&mut dirs);
    }
    
    entries
}

pub fn load_template_files(template_root: &PathBuf) -> Vec<PathBuf> {
    WalkDir::new(template_root)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.into_path())
        .collect()
}