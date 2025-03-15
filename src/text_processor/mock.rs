use std::path::Path;
use std::collections::HashMap;
use super::TextProcessing;

/// Mock implementation of TextProcessing for testing
pub struct MockTextProcessor {
    text_files: HashMap<String, String>,
    binary_files: Vec<String>,
}

impl MockTextProcessor {
    pub fn new() -> Self {
        Self {
            text_files: HashMap::new(),
            binary_files: Vec::new(),
        }
    }

    /// Add a mock text file
    pub fn add_text_file(&mut self, path: &str, content: &str) {
        self.text_files.insert(path.to_string(), content.to_string());
    }

    /// Add a mock binary file
    pub fn add_binary_file(&mut self, path: &str) {
        self.binary_files.push(path.to_string());
    }
}

impl TextProcessing for MockTextProcessor {
    fn process_file(&self, path: &Path) -> Result<Option<String>, String> {
        let path_str = path.to_string_lossy().to_string();
        
        if self.binary_files.contains(&path_str) {
            return Ok(None);
        }
        
        if let Some(content) = self.text_files.get(&path_str) {
            Ok(Some(content.clone()))
        } else {
            // Default to treating unknown files as text with empty content for simplicity
            Ok(Some(String::new()))
        }
    }

    fn format_text_content(&self, path: &Path, content: &str, buffer: &mut String) -> Result<bool, String> {
        // Simple implementation for testing
        buffer.push_str(&format!("=== {} ===\n", path.display()));
        buffer.push_str(content);
        buffer.push_str("\n\n");
        
        Ok(true)
    }
} 