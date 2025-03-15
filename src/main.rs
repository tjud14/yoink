mod cli;
mod clipboard;
mod file_processor;
mod file_scanner;
mod file_tree;
mod text_processor;
mod utils;

use colored::*;
use file_processor::FileProcessor;

fn main() {
    let matches = cli::build_cli().get_matches();
    let mut config = cli::Config::from_matches(&matches);
    
    // Expand any environment variables and tilde (~) in the path
    match shellexpand::full(&config.path) {
        Ok(expanded_path) => {
            config.path = expanded_path.into_owned();
        },
        Err(e) => {
            eprintln!("{}: Failed to expand path '{}': {}", "Error".red(), config.path, e);
            std::process::exit(1);
        }
    }
    
    // Create processor with default dependencies using the factory method
    let mut processor = FileProcessor::with_defaults(config);
    
    match processor.process() {
        Ok((text_count, binary_count)) => {
            if text_count == 0 && binary_count == 0 {
                println!("{}", "No files found".yellow());
                return;
            }
            
            if text_count > 0 {
                println!(
                    "{} {} {} {}",
                    "✨".green(),
                    "Yoinked".green().bold(),
                    text_count,
                    if text_count == 1 { "text file!" } else { "text files!" }.green()
                );
            }
            
            if binary_count > 0 {
                println!(
                    "{} {} {}",
                    "📊".yellow(),
                    binary_count,
                    if binary_count == 1 { "binary file was skipped" } else { "binary files were skipped" }.yellow()
                );
            }
            
            println!("{} Content copied to clipboard", "📋".cyan());
        }
        Err(e) => {
            eprintln!("{}: {}", "Error".red(), e);
            std::process::exit(1);
        }
    }
}