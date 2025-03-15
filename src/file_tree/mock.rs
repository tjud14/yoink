use super::DirectoryTreeBuilding;

/// Mock implementation of DirectoryTreeBuilding for testing
pub struct MockDirectoryTreeBuilder {
    mock_tree: String,
}

impl MockDirectoryTreeBuilder {
    pub fn new() -> Self {
        Self {
            mock_tree: String::new(),
        }
    }

    /// Set a predefined directory tree structure for testing
    pub fn set_mock_tree(&mut self, tree: &str) {
        self.mock_tree = tree.to_string();
    }
}

impl DirectoryTreeBuilding for MockDirectoryTreeBuilder {
    fn build_directory_tree(&self, buffer: &mut String) -> Result<(), String> {
        // Just append the predefined mock tree structure
        buffer.push_str(&self.mock_tree);
        Ok(())
    }
} 