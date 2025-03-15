use crate::cli::Config;
use crate::clipboard::ClipboardInterface;
use crate::file_tree::DirectoryTreeBuilding;
use crate::file_scanner::{FileScanning, FileEntry};
use crate::text_processor::TextProcessing;
use indicatif::{ProgressBar, ProgressStyle};
use std::sync::{Arc, Mutex};
use rayon::prelude::*;

pub struct FileProcessor {
    config: Config,
    clipboard: Box<dyn ClipboardInterface>,
    file_scanner: Box<dyn FileScanning>,
    text_processor: Box<dyn TextProcessing>,
    dir_tree_builder: Box<dyn DirectoryTreeBuilding>,
}

impl FileProcessor {
    pub fn new(
        config: Config,
        clipboard: Box<dyn ClipboardInterface>,
        file_scanner: Box<dyn FileScanning>,
        text_processor: Box<dyn TextProcessing>,
        dir_tree_builder: Box<dyn DirectoryTreeBuilding>,
    ) -> Self {
        Self {
            config,
            clipboard,
            file_scanner,
            text_processor,
            dir_tree_builder,
        }
    }

    /// Factory method to create FileProcessor with default dependencies
    pub fn with_defaults(config: Config) -> Self {
        use crate::clipboard::ClipboardManager;
        use crate::file_tree::DirectoryTreeBuilder;
        use crate::file_scanner::FileScanner;
        use crate::text_processor::TextProcessor;
        
        Self {
            clipboard: Box::new(ClipboardManager::new(config.verbose)),
            file_scanner: Box::new(FileScanner::new(&config)),
            text_processor: Box::new(TextProcessor::new(&config)),
            dir_tree_builder: Box::new(DirectoryTreeBuilder::new(&config)),
            config,
        }
    }

    pub fn process(&mut self) -> Result<(usize, usize), String> {
        let pb = self.setup_progress_bar();
        
        // Create thread-safe buffer and counters
        let buffer = Arc::new(Mutex::new(String::new()));
        let text_count = Arc::new(Mutex::new(0));
        let binary_count = Arc::new(Mutex::new(0));
        
        // Add directory structure at the top
        {
            let mut buffer = buffer.lock().unwrap();
            buffer.push_str("=== DIRECTORY STRUCTURE ===\n");
            self.dir_tree_builder.build_directory_tree(&mut buffer)?;
            buffer.push_str("\n=== TEXT FILES ===\n\n");
        }

        // Collect and filter files first
        let mut entries = self.file_scanner.collect_files();
        
        if self.config.sort {
            entries.sort_by_key(|e| e.path().to_path_buf());
        }
        
        // Setup progress tracking
        let progress = self.setup_file_progress(entries.len());
        
        // Process files in parallel
        entries.par_iter().for_each(|entry| {
            let buffer = Arc::clone(&buffer);
            let text_count = Arc::clone(&text_count);
            let binary_count = Arc::clone(&binary_count);
            let progress = Arc::clone(&progress);
            
            // Process each file
            if let Err(e) = self.process_file_parallel(
                entry, 
                &buffer, 
                &progress, 
                &text_count, 
                &binary_count
            ) {
                let mut progress = progress.lock().unwrap();
                progress.println(format!("Error processing file {}: {}", entry.path().display(), e));
            }
            
            // Increment progress bar
            let mut progress = progress.lock().unwrap();
            progress.inc(1);
        });
        
        // Finalize the output
        {
            let mut buffer = buffer.lock().unwrap();
            buffer.push_str("\n=== SUMMARY ===\n");
            let text_count = *text_count.lock().unwrap();
            let binary_count = *binary_count.lock().unwrap();
            buffer.push_str(&format!("Text files processed: {}\n", text_count));
            buffer.push_str(&format!("Binary files skipped: {}\n", binary_count));
            
            // Copy to clipboard
            progress.lock().unwrap().finish_and_clear();
            self.clipboard.copy_to_clipboard(&buffer)?;
            
            Ok((text_count, binary_count))
        }
    }

    // This function processes a single file in parallel
    fn process_file_parallel(
        &self,
        entry: &walkdir::DirEntry,
        buffer: &Arc<Mutex<String>>,
        progress: &Arc<Mutex<ProgressBar>>,
        text_count: &Arc<Mutex<usize>>,
        binary_count: &Arc<Mutex<usize>>,
    ) -> Result<(), String> {
        let path = entry.path();
        
        // Skip if not a file
        if !path.is_file() {
            return Ok(());
        }
        
        // Check file size
        let metadata = match path.metadata() {
            Ok(metadata) => metadata,
            Err(e) => {
                return Err(format!("Failed to get metadata for {}: {}", path.display(), e));
            }
        };
        
        if metadata.len() > self.config.max_size {
            if self.config.verbose {
                progress.lock().unwrap().println(
                    format!("Skipping large file: {} ({} bytes)", path.display(), metadata.len())
                );
            }
            return Ok(());
        }
        
        // Process the file based on its type
        let result = self.text_processor.process_file(path);
        
        match result {
            Ok(Some(content)) => {
                // Update the buffer with the processed text content
                let mut buffer = buffer.lock().unwrap();
                let was_included = self.text_processor.format_text_content(path, &content, &mut buffer)?;
                
                if was_included {
                    // Increment text count
                    let mut text_count = text_count.lock().unwrap();
                    *text_count += 1;
                    
                    if self.config.verbose {
                        progress.lock().unwrap().println(
                            format!("Processed text file: {}", path.display())
                        );
                    }
                }
            }
            Ok(None) => {
                // It's a binary file or we're skipping it
                let mut binary_count = binary_count.lock().unwrap();
                *binary_count += 1;
                
                if self.config.verbose {
                    progress.lock().unwrap().println(
                        format!("Skipping binary file: {}", path.display())
                    );
                }
            }
            Err(e) => {
                return Err(format!("Error processing {}: {}", path.display(), e));
            }
        }
        
        Ok(())
    }

    fn setup_progress_bar(&self) -> ProgressBar {
        // Create a progress bar with a spinner for the initial phase
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap()
                .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
        );
        pb.set_message("Scanning files...");
        pb.enable_steady_tick(std::time::Duration::from_millis(80));
        pb
    }

    fn setup_file_progress(&self, file_count: usize) -> Arc<Mutex<ProgressBar>> {
        // Create a progress bar that tracks the number of files
        let progress_style = ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} files ({eta})")
            .unwrap()
            .progress_chars("#>-");
            
        Arc::new(Mutex::new(
            ProgressBar::new(file_count as u64)
                .with_style(progress_style)
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clipboard::MockClipboardManager;
    use crate::file_scanner::MockFileScanner;
    use crate::text_processor::MockTextProcessor;
    use crate::file_tree::MockDirectoryTreeBuilder;
    use std::path::PathBuf;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_processor_with_mocks() {
        // Create a mock config
        let config = crate::cli::Config {
            path: "/mock/path".to_string(),
            max_size: 1024 * 1024, // 1MB
            verbose: false,
            max_depth: 1,
            include_extensions: None,
            exclude_extensions: None,
            exclude_paths: None,
            pattern: None,
            skip_hidden: false,
            sort: false,
            save_config: false,
            search_text: None,
            case_sensitive: false,
        };
        
        // Create mock components
        let mock_clipboard = MockClipboardManager::new(false);
        let mut mock_file_scanner = MockFileScanner::new();
        let mut mock_text_processor = MockTextProcessor::new();
        let mut mock_dir_tree_builder = MockDirectoryTreeBuilder::new();
        
        // Setup mock directory tree
        mock_dir_tree_builder.set_mock_tree("📁 mock/\n  📄 test.txt\n");
        
        // Setup mock text processor
        mock_text_processor.add_text_file("/mock/path/test.txt", "This is test content");
        
        // Create the processor with mocked dependencies
        let mut processor = FileProcessor::new(
            config,
            Box::new(mock_clipboard),
            Box::new(mock_file_scanner),
            Box::new(mock_text_processor),
            Box::new(mock_dir_tree_builder),
        );
        
        // Process the mock files
        let result = processor.process();
        
        // Since we're using empty mock file scanner that returns no files,
        // expect zero processed files
        assert!(result.is_ok());
        let (text_count, binary_count) = result.unwrap();
        assert_eq!(text_count, 0);
        assert_eq!(binary_count, 0);
    }
}