use crate::cli::Config;
use walkdir::WalkDir;
use std::path::PathBuf;
use super::DirectoryTreeBuilding;

pub struct DirectoryTreeBuilder {
    config: Config,
}

impl DirectoryTreeBuilder {
    pub fn new(config: &Config) -> Self {
        Self {
            config: config.clone(),
        }
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
}

impl DirectoryTreeBuilding for DirectoryTreeBuilder {
    fn build_directory_tree(&self, buffer: &mut String) -> Result<(), String> {
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
} 