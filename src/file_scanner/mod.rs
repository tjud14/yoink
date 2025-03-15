pub mod scanner;
#[cfg(test)]
pub mod mock;

// Re-export the implementation
pub use scanner::FileScanner;
pub use walkdir::DirEntry as FileEntry;
#[cfg(test)]
pub use mock::MockFileScanner;

/// Trait defining the file scanning operations interface
pub trait FileScanning {
    /// Collect files from the specified path according to filters
    fn collect_files(&self) -> Vec<FileEntry>;
} 