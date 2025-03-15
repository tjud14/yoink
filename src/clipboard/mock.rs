use std::cell::RefCell;
use super::ClipboardInterface;

/// Mock implementation of ClipboardInterface for testing
pub struct MockClipboardManager {
    verbose: bool,
    copied_text: RefCell<Option<String>>,
}

impl MockClipboardManager {
    pub fn new(verbose: bool) -> Self {
        Self {
            verbose,
            copied_text: RefCell::new(None),
        }
    }

    /// Get the text that was "copied" to the clipboard
    pub fn get_copied_text(&self) -> Option<String> {
        self.copied_text.borrow().clone()
    }
}

impl ClipboardInterface for MockClipboardManager {
    fn copy_to_clipboard(&self, text: &str) -> Result<(), String> {
        // Store the text instead of actually copying to clipboard
        *self.copied_text.borrow_mut() = Some(text.to_string());
        
        if self.verbose {
            println!("Mock clipboard: text copied (length: {})", text.len());
        }
        
        Ok(())
    }
} 