use crate::cli::Config;
use crate::clipboard::ClipboardManager;
use crate::utils::is_text;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use walkdir::WalkDir;
use std::path::PathBuf;

pub struct FileProcessor {
    config: Config,
    clipboard: ClipboardManager,
}

impl FileProcessor {
    pub fn new(config: Config) -> Self {
        Self {
            clipboard: ClipboardManager::new(config.verbose),
            config,
        }
    }

    pub fn process(&mut self) -> Result<(usize, usize), String> {
        let pb = self.setup_progress_bar();
        let mut buffer = String::new();
        let mut text_count = 0;
        let mut binary_count = 0;

        // Add directory structure at the top
        buffer.push_str("=== DIRECTORY STRUCTURE ===\n");
        self.add_directory_structure(&mut buffer)?;
        buffer.push_str("\n=== TEXT FILES ===\n\n");

        // Collect and filter files
        let mut entries = self.collect_files();
        
        if self.config.sort {
            entries.sort_by_key(|e| e.path().to_path_buf());
        }

        for entry in entries {
            self.process_file(entry, &mut buffer, &pb, &mut text_count, &mut binary_count)?;
        }

        buffer.push_str("\n=== SUMMARY ===\n");
        buffer.push_str(&format!("Text files processed: {}\n", text_count));
        buffer.push_str(&format!("Binary files found: {}\n", binary_count));

        pb.finish_and_clear();
        self.clipboard.copy_to_clipboard(&buffer)?;

        Ok((text_count, binary_count))
    }

    fn add_directory_structure(&self, buffer: &mut String) -> Result<(), String> {
        // Create a PathBuf to handle special characters properly
        let base_path = PathBuf::from(&self.config.path);
        
        // Check if path exists before processing
        if !base_path.exists() {
            return Err(format!("Path does not exist: {}", base_path.display()));
        }
        
        let entries: Vec<_> = WalkDir::new(&base_path)
            .into_iter()
            .filter_map(|e| {
                match e {
                    Ok(entry) => {
                        if self.should_include_in_tree(&entry) {
                            Some(entry)
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
            .collect();

        // Sort entries to get a consistent tree view
        let mut sorted_entries = entries;
        sorted_entries.sort_by_key(|e| e.path().to_path_buf());

        for entry in sorted_entries {
            let depth = entry.depth();
            let indent = "  ".repeat(depth);
            let name = entry.file_name().to_string_lossy();

            if entry.file_type().is_dir() {
                buffer.push_str(&format!("{}📁 {}/\n", indent, name));
            } else {
                buffer.push_str(&format!("{}📄 {}\n", indent, name));
            }
        }

        Ok(())
    }

    fn should_include_in_tree(&self, entry: &walkdir::DirEntry) -> bool {
        if self.config.skip_hidden && entry.file_name().to_string_lossy().starts_with('.') {
            return false;
        }

        if let Some(ref exclude_paths) = self.config.exclude_paths {
            let path_str = entry.path().to_string_lossy();
            
            if exclude_paths.iter().any(|excluded| {
                path_str.split('/').any(|component| component == excluded)
            }) {
                return false;
            }
        }

        true
    }

    fn setup_progress_bar(&self) -> ProgressBar {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap(),
        );
        pb.set_message("Scanning files...");
        pb
    }

    fn process_file(
        &self,
        entry: walkdir::DirEntry,
        buffer: &mut String,
        pb: &ProgressBar,
        text_count: &mut usize,
        binary_count: &mut usize,
    ) -> Result<(), String> {
        let file_path = entry.path();
        let file_size = entry.metadata().map(|m| m.len()).unwrap_or(0);
        
        if file_size > self.config.max_size {
            if self.config.verbose {
                pb.println(format!("Skipping large file: {}", file_path.display()));
            }
            return Ok(());
        }

        match fs::read(file_path) {
            Ok(content) => {
                if !is_text(&content) {
                    if self.config.verbose {
                        pb.println(format!("Binary found: {}", file_path.display()));
                    }
                    buffer.push_str(&format!("BINARY: {}\n", file_path.display()));
                    *binary_count += 1;
                } else {
                    if self.config.verbose {
                        pb.println(format!("Processing text: {}", file_path.display()));
                    }
                    
                    buffer.push_str(&format!("\n=== {} ===\n", file_path.display()));
                    if let Ok(content_str) = String::from_utf8(content) {
                        buffer.push_str(&content_str);
                        buffer.push_str("\n");
                        *text_count += 1;
                    }
                }
            }
            Err(e) => {
                if self.config.verbose {
                    pb.println(format!("Error reading {}: {}", file_path.display(), e));
                }
            }
        }

        Ok(())
    }

    fn collect_files(&self) -> Vec<walkdir::DirEntry> {
        // Use PathBuf to properly handle special characters
        let path = std::path::PathBuf::from(&self.config.path);
        
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

    fn should_process_file(&self, entry: &walkdir::DirEntry) -> bool {
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