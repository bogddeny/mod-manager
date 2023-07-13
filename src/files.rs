use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub fn read_directory_folders(path_to_directory: &str) -> Vec<PathBuf> {
    let folders = std::fs::read_dir(path_to_directory)
        .expect("Failed to read directory")
        .filter_map(Result::ok)
        .filter(|entry| {
            entry
                .file_type()
                .ok()
                .map(|ft| ft.is_dir())
                .unwrap_or(false)
        })
        .map(|entry| entry.path())
        .collect::<Vec<_>>();

    return folders;
}

pub fn read_meta_properties(mod_directory: &PathBuf) -> Option<String> {
    let meta_file_path = mod_directory.join("meta.ini");
    let mut version = None;

    if let Ok(file) = File::open(meta_file_path) {
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                if line.starts_with("version=") {
                    version = Some(line.trim_start_matches("version=").to_string());
                    break;
                }
            }
        }
    }
    return version;
}
