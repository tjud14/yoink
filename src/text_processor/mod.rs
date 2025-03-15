pub mod processor;
#[cfg(test)]
pub mod mock;

// Re-export the implementation
pub use processor::TextProcessor;
#[cfg(test)]
pub use mock::MockTextProcessor;

use std::path::Path;

/// Trait defining the text processing operations interface
pub trait TextProcessing {
    /// Process a file and determine if it's a text file, returning its content if so
    fn process_file(&self, path: &Path) -> Result<Option<String>, String>;
    
    /// Format text content for display/clipboard and return whether it was included
    fn format_text_content(&self, path: &Path, content: &str, buffer: &mut String) -> Result<bool, String>;
} 