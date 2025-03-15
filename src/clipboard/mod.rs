pub mod manager;
#[cfg(test)]
pub mod mock;

// Re-export the implementation
pub use manager::ClipboardManager;
#[cfg(test)]
pub use mock::MockClipboardManager;

/// Trait defining the clipboard operations interface
pub trait ClipboardInterface {
    /// Copy text to the system clipboard
    fn copy_to_clipboard(&self, text: &str) -> Result<(), String>;
} 