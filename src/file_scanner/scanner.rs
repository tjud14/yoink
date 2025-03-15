use crate::cli::Config;
use walkdir::WalkDir;
use std::path::PathBuf;
use super::{FileScanning, FileEntry};

pub struct FileScanner {
    config: Config,
}

impl FileScanner {
    pub fn new(config: &Config) -> Self {
        Self {
            config: config.clone(),
        }
    }

    fn should_process_file(&self, entry: &FileEntry) -> bool {
        if self.config.skip_hidden && entry.file_name().to_string_lossy().starts_with('.') {
            if self.config.verbose {
                println!("Skipping hidden file: {}", entry.path().display());
            }
            return false;
        }

        if let Some(ref exclude_paths) = self.config.exclude_paths {
            let path_str = entry.path().to_string_lossy();
            
            // Use literal path component comparison
            if exclude_paths.iter().any(|excluded| {
                // Compare path components to avoid partial matching issues
                path_str.split('/').any(|component| component == excluded)
            }) {
                if self.config.verbose {
                    println!("Skipping excluded path: {}", entry.path().display());
                }
                return false;
            }
        }

        let extension = entry.path()
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase());

        if let Some(ref include_exts) = self.config.include_extensions {
            if extension
                .as_ref()
                .map(|ext| !include_exts.contains(ext))
                .unwrap_or(true) {
                    if self.config.verbose {
                        println!("Skipping non-included extension: {}", entry.path().display());
                    }
                    return false;
                }
        }

        if let Some(ref exclude_exts) = self.config.exclude_extensions {
            if extension
                .as_ref()
                .map(|ext| exclude_exts.contains(ext))
                .unwrap_or(false) {
                    if self.config.verbose {
                        println!("Skipping excluded extension: {}", entry.path().display());
                    }
                    return false;
                }
        }

        if let Some(ref pattern) = self.config.pattern {
            let filename = entry.path()
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("");
                
            if !pattern.matches(filename) {
                if self.config.verbose {
                    println!("Skipping non-matching pattern: {}", entry.path().display());
                }
                return false;
            }
        }

        true
    }
}

impl FileScanning for FileScanner {
    fn collect_files(&self) -> Vec<FileEntry> {
        // Use PathBuf to properly handle special characters
        let path = PathBuf::from(&self.config.path);
        
        // Check if path exists before walking
        if !path.exists() {
            if self.config.verbose {
                eprintln!("Path does not exist: {}", path.display());
            }
            return Vec::new();
        }
        
        WalkDir::new(path)
            .max_depth(self.config.max_depth as usize)
            .follow_links(false)
            .into_iter()
            .filter_map(|entry| {
                match entry {
                    Ok(e) => {
                        if !e.file_type().is_dir() && self.should_process_file(&e) {
                            Some(e)
                        } else {
                            None
                        }
                    },
                    Err(err) => {
                        if self.config.verbose {
                            eprintln!("Error accessing path: {}", err);
                        }
                        None
                    }
                }
            })
            .collect()
    }
} 