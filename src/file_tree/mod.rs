pub mod builder;
#[cfg(test)]
pub mod mock;

// Re-export the implementation
pub use builder::DirectoryTreeBuilder;
#[cfg(test)]
pub use mock::MockDirectoryTreeBuilder;

/// Trait defining the directory tree building operations interface
pub trait DirectoryTreeBuilding {
    /// Build a text representation of the directory tree structure
    fn build_directory_tree(&self, buffer: &mut String) -> Result<(), String>;
} 