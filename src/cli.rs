use clap::{Command, Arg};
use std::fs;
use std::path::PathBuf;
use std::io::{Read, Write};
use colored::*;

#[derive(Clone)]
pub struct Config {
    pub path: String,
    pub max_size: u64,
    pub verbose: bool,
    pub max_depth: u32,
    pub include_extensions: Option<Vec<String>>,
    pub exclude_extensions: Option<Vec<String>>,
    pub exclude_paths: Option<Vec<String>>,
    pub pattern: Option<glob::Pattern>,
    pub skip_hidden: bool,
    pub sort: bool,
    pub save_config: bool,
    pub search_text: Option<String>,
    pub case_sensitive: bool,
}

impl Config {
    pub fn from_matches(matches: &clap::ArgMatches) -> Self {
        // Try to load config file first
        let mut config = if matches.get_flag("no-config") {
            Self::default()
        } else {
            Self::load_from_file().unwrap_or_else(|_| Self::default())
        };
        
        // Override with command line arguments
        if matches.contains_id("path") {
            config.path = matches.get_one::<String>("path").unwrap().clone();
        }
        
        if matches.contains_id("max-size") {
            config.max_size = matches.get_one::<String>("max-size")
                .unwrap()
                .parse::<u64>()
                .unwrap_or(10) * 1024 * 1024;
        }
        
        if matches.get_flag("verbose") {
            config.verbose = true;
        }
        
        if matches.contains_id("depth") {
            config.max_depth = matches.get_one::<String>("depth")
                .and_then(|d| d.parse::<u32>().ok())
                .unwrap_or(u32::MAX);
        }
        
        if matches.contains_id("extensions") {
            config.include_extensions = matches.get_one::<String>("extensions")
                .map(|e| e.split(',').map(|s| s.trim().to_lowercase()).collect());
        }
        
        if matches.contains_id("exclude") {
            config.exclude_extensions = matches.get_one::<String>("exclude")
                .map(|e| e.split(',').map(|s| s.trim().to_lowercase()).collect());
        }
        
        if matches.contains_id("exclude-paths") {
            config.exclude_paths = matches.get_one::<String>("exclude-paths")
                .map(|p| p.split(',').map(|s| s.trim().to_string()).collect());
        }
        
        if matches.contains_id("pattern") {
            config.pattern = matches.get_one::<String>("pattern").map(|p| {
                match glob::Pattern::new(p) {
                    Ok(pattern) => pattern,
                    Err(e) => {
                        eprintln!("Warning: Invalid pattern '{}': {}. Treating as literal.", p, e);
                        let escaped = p.chars().flat_map(|c| {
                            match c {
                                '*' | '?' | '[' | ']' | '{' | '}' | '(' | ')' => vec!['\\', c],
                                _ => vec![c],
                            }
                        }).collect::<String>();
                        glob::Pattern::new(&escaped).unwrap_or_else(|_| {
                            glob::Pattern::new("NOMATCH").unwrap()
                        })
                    }
                }
            });
        }
        
        if matches.get_flag("no-hidden") {
            config.skip_hidden = true;
        }
        
        if matches.get_flag("sort") {
            config.sort = true;
        }
        
        if matches.contains_id("search") {
            config.search_text = matches.get_one::<String>("search").map(|s| s.to_string());
        }
        
        if matches.get_flag("case-sensitive") {
            config.case_sensitive = true;
        }
        
        config.save_config = matches.get_flag("save-config");
        
        // Save config if requested
        if config.save_config {
            if let Err(e) = config.save_to_file() {
                eprintln!("{}: Failed to save config: {}", "Warning".yellow(), e);
            } else {
                println!("{}: Configuration saved", "Info".blue());
            }
        }
        
        config
    }
    
    fn default() -> Self {
        Self {
            path: ".".to_string(),
            max_size: 10 * 1024 * 1024,
            verbose: false,
            max_depth: u32::MAX,
            include_extensions: None,
            exclude_extensions: None,
            exclude_paths: None,
            pattern: None,
            skip_hidden: false,
            sort: false,
            save_config: false,
            search_text: None,
            case_sensitive: false,
        }
    }
    
    fn get_config_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("yoink");
        fs::create_dir_all(&path).ok();
        path.push("config.json");
        path
    }
    
    fn save_to_file(&self) -> Result<(), String> {
        let config_path = Self::get_config_path();
        
        // Create a serializable version of the config
        let serializable_config = serde_json::json!({
            "path": self.path,
            "max_size": self.max_size / (1024 * 1024), // Convert back to MB
            "verbose": self.verbose,
            "max_depth": if self.max_depth == u32::MAX { null } else { self.max_depth },
            "include_extensions": self.include_extensions,
            "exclude_extensions": self.exclude_extensions,
            "exclude_paths": self.exclude_paths,
            "pattern": self.pattern.as_ref().map(|p| p.as_str()),
            "skip_hidden": self.skip_hidden,
            "sort": self.sort,
            "search_text": self.search_text,
            "case_sensitive": self.case_sensitive,
        });
        
        let config_str = serde_json::to_string_pretty(&serializable_config)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;
        
        let mut file = fs::File::create(config_path)
            .map_err(|e| format!("Failed to create config file: {}", e))?;
        
        file.write_all(config_str.as_bytes())
            .map_err(|e| format!("Failed to write config file: {}", e))?;
        
        Ok(())
    }
    
    fn load_from_file() -> Result<Self, String> {
        let config_path = Self::get_config_path();
        
        if !config_path.exists() {
            return Err("Config file does not exist".to_string());
        }
        
        let mut file = fs::File::open(config_path)
            .map_err(|e| format!("Failed to open config file: {}", e))?;
        
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| format!("Failed to read config file: {}", e))?;
        
        let json: serde_json::Value = serde_json::from_str(&contents)
            .map_err(|e| format!("Failed to parse config file: {}", e))?;
        
        let mut config = Self::default();
        
        if let Some(path) = json.get("path").and_then(|v| v.as_str()) {
            config.path = path.to_string();
        }
        
        if let Some(max_size) = json.get("max_size").and_then(|v| v.as_u64()) {
            config.max_size = max_size * 1024 * 1024; // Convert from MB
        }
        
        if let Some(verbose) = json.get("verbose").and_then(|v| v.as_bool()) {
            config.verbose = verbose;
        }
        
        if let Some(max_depth) = json.get("max_depth").and_then(|v| v.as_u64()) {
            config.max_depth = max_depth as u32;
        }
        
        if let Some(extensions) = json.get("include_extensions") {
            if let Some(arr) = extensions.as_array() {
                let exts: Vec<String> = arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect();
                if !exts.is_empty() {
                    config.include_extensions = Some(exts);
                }
            }
        }
        
        if let Some(exclude) = json.get("exclude_extensions") {
            if let Some(arr) = exclude.as_array() {
                let exts: Vec<String> = arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect();
                if !exts.is_empty() {
                    config.exclude_extensions = Some(exts);
                }
            }
        }
        
        if let Some(exclude_paths) = json.get("exclude_paths") {
            if let Some(arr) = exclude_paths.as_array() {
                let paths: Vec<String> = arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect();
                if !paths.is_empty() {
                    config.exclude_paths = Some(paths);
                }
            }
        }
        
        if let Some(pattern_str) = json.get("pattern").and_then(|v| v.as_str()) {
            if let Ok(pattern) = glob::Pattern::new(pattern_str) {
                config.pattern = Some(pattern);
            }
        }
        
        if let Some(skip_hidden) = json.get("skip_hidden").and_then(|v| v.as_bool()) {
            config.skip_hidden = skip_hidden;
        }
        
        if let Some(sort) = json.get("sort").and_then(|v| v.as_bool()) {
            config.sort = sort;
        }
        
        if let Some(search_text) = json.get("search_text").and_then(|v| v.as_str()) {
            config.search_text = Some(search_text.to_string());
        }
        
        if let Some(case_sensitive) = json.get("case_sensitive").and_then(|v| v.as_bool()) {
            config.case_sensitive = case_sensitive;
        }
        
        Ok(config)
    }
}

pub fn build_cli() -> Command {
    Command::new("yoink")
        .version("0.1.0")
        .about("Quickly grab text content into your clipboard")
        .arg(
            Arg::new("path")
                .help("Directory or file to yoink")
                .default_value(".")
                .index(1)
        )
        .arg(
            Arg::new("max-size")
                .short('m')
                .long("max-size")
                .value_name("SIZE")
                .default_value("10")
                .help("Maximum file size in MB to consider")
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(clap::ArgAction::SetTrue)
                .help("Show verbose output")
        )
        .arg(
            Arg::new("depth")
                .short('d')
                .long("depth")
                .value_name("DEPTH")
                .help("Maximum directory depth to traverse (0 means current directory only)")
        )
        .arg(
            Arg::new("extensions")
                .short('e')
                .long("extensions")
                .value_name("EXTS")
                .help("File extensions to include (comma-separated, e.g., \"txt,md,rs\")")
        )
        .arg(
            Arg::new("exclude")
                .short('x')
                .long("exclude")
                .value_name("EXTS")
                .help("File extensions to exclude (comma-separated)")
        )
        .arg(
            Arg::new("exclude-paths")
                .long("exclude-paths")
                .value_name("PATHS")
                .help("Paths to exclude (comma-separated, exact names, not patterns)")
        )
        .arg(
            Arg::new("pattern")
                .short('p')
                .long("pattern")
                .value_name("PATTERN")
                .help("Search pattern for filenames (supports glob patterns like *.txt, special chars like () need escaping with \\)")
        )
        .arg(
            Arg::new("no-hidden")
                .short('H')
                .long("no-hidden")
                .action(clap::ArgAction::SetTrue)
                .help("Skip hidden files and directories")
        )
        .arg(
            Arg::new("sort")
                .short('s')
                .long("sort")
                .action(clap::ArgAction::SetTrue)
                .help("Sort files by name before processing")
        )
        .arg(
            Arg::new("save-config")
                .long("save-config")
                .action(clap::ArgAction::SetTrue)
                .help("Save current configuration as default")
        )
        .arg(
            Arg::new("no-config")
                .long("no-config")
                .action(clap::ArgAction::SetTrue)
                .help("Ignore saved configuration file")
        )
        .arg(
            Arg::new("search")
                .short('S')
                .long("search")
                .value_name("TEXT")
                .help("Search for text content within files")
        )
        .arg(
            Arg::new("case-sensitive")
                .short('c')
                .long("case-sensitive")
                .action(clap::ArgAction::SetTrue)
                .help("Make text search case-sensitive")
        )
}