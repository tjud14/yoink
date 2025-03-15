use crate::cli::Config;
use crate::utils::{is_text, is_text_file};
use super::TextProcessing;
use std::fs;
use std::path::Path;

pub struct TextProcessor {
    config: Config,
}

impl TextProcessor {
    pub fn new(config: &Config) -> Self {
        Self {
            config: config.clone(),
        }
    }
}

impl TextProcessing for TextProcessor {
    fn process_file(&self, path: &Path) -> Result<Option<String>, String> {
        // First try to determine if it's a text file by extension and content type
        let is_text_result = is_text_file(path);
         
        match is_text_result {
            Ok(true) => {
                // It's a text file, read and process its content
                match fs::read_to_string(path) {
                    Ok(content) => Ok(Some(content)),
                    Err(e) => {
                        if self.config.verbose {
                            println!("Error reading text file {}: {}", path.display(), e);
                        }
                        Ok(None)
                    }
                }
            },
            Ok(false) => {
                // It's a binary file
                Ok(None)
            },
            Err(e) => {
                // Error determining file type, use legacy method as fallback
                if self.config.verbose {
                    println!("Warning: Could not determine file type, falling back to content analysis: {}", e);
                }
                
                // Read file content
                match fs::read(path) {
                    Ok(content) => {
                        // Check if it's a text file using the legacy method
                        if is_text(&content) {
                            // Convert to string
                            match String::from_utf8(content) {
                                Ok(text) => Ok(Some(text)),
                                Err(_) => {
                                    if self.config.verbose {
                                        println!("Error converting file to UTF-8: {}", path.display());
                                    }
                                    Ok(None)
                                }
                            }
                        } else {
                            Ok(None)
                        }
                    },
                    Err(e) => {
                        if self.config.verbose {
                            println!("Error reading file {}: {}", path.display(), e);
                        }
                        Ok(None)
                    }
                }
            }
        }
    }

    fn format_text_content(&self, path: &Path, content: &str, buffer: &mut String) -> Result<bool, String> {
        // Check if we need to search for text
        if let Some(search_text) = &self.config.search_text {
            let found = if self.config.case_sensitive {
                content.contains(search_text)
            } else {
                content.to_lowercase().contains(&search_text.to_lowercase())
            };
            
            if !found {
                return Ok(false);
            }
            
            // Add search match information
            buffer.push_str(&format!("=== MATCH IN: {} ===\n", path.display()));
            
            // Add context around matches
            let lines: Vec<&str> = content.lines().collect();
            let mut found_lines = Vec::new();
            
            for (i, line) in lines.iter().enumerate() {
                let line_matches = if self.config.case_sensitive {
                    line.contains(search_text)
                } else {
                    line.to_lowercase().contains(&search_text.to_lowercase())
                };
                
                if line_matches {
                    // Add context (3 lines before and after)
                    let start = i.saturating_sub(3);
                    let end = (i + 3).min(lines.len() - 1);
                    
                    for j in start..=end {
                        found_lines.push((j, lines[j]));
                    }
                    
                    // Add a separator between different match contexts
                    found_lines.push((usize::MAX, "..."));
                }
            }
            
            // Remove duplicates and sort
            found_lines.sort_by_key(|&(idx, _)| idx);
            found_lines.dedup_by_key(|&mut (idx, _)| idx);
            
            // Add to buffer
            let mut prev_idx = 0;
            let mut first = true;
            
            for (idx, line) in found_lines {
                if idx == usize::MAX {
                    buffer.push_str("...\n");
                    first = true;
                    continue;
                }
                
                if !first && idx > prev_idx + 1 {
                    buffer.push_str("...\n");
                }
                
                // Add the line with line number
                buffer.push_str(&format!("{}: {}\n", idx + 1, line));
                
                prev_idx = idx;
                first = false;
            }
            
            buffer.push_str("\n");
        } else {
            // Add file header
            buffer.push_str(&format!("=== {} ===\n", path.display()));
            buffer.push_str(content);
            buffer.push_str("\n\n");
        }
        
        Ok(true)
    }
} 