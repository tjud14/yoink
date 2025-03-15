use std::path::{Path, PathBuf};
use walkdir::DirEntry;
use super::{FileScanning, FileEntry};

/// Mock implementation of FileScanning for testing
pub struct MockFileScanner {
    files: Vec<PathBuf>,
}

impl MockFileScanner {
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
        }
    }

    /// Add a mock file to the scanner
    pub fn add_file(&mut self, path: PathBuf) {
        self.files.push(path);
    }
}

impl FileScanning for MockFileScanner {
    fn collect_files(&self) -> Vec<FileEntry> {
        // This is a simplified mock implementation that doesn't actually
        // create real DirEntry objects, since they're hard to construct.
        // In real tests, you might want to use tempfile to create actual files.
        vec![]
    }
} 