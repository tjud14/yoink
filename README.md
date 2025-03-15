# Yoink

Yoink is a command-line utility that quickly grabs text content from files and directories and copies it to your clipboard. It's perfect for quickly collecting code snippets, documentation, or any text content you need to share or analyze.

## Features

- 📋 Copy text content from files to your clipboard
- 🔍 Search for specific text within files
- 📁 Include directory structure in the output
- 🔎 Filter files by extension, pattern, or size
- 💾 Save your configuration for future use
- 🖥️ Cross-platform clipboard support (Linux, macOS, Windows)
- 🧠 Smart file type detection
- ⚡ Parallel file processing for improved performance

## Advanced Features

### Smart File Type Detection

Yoink uses a sophisticated approach to determine whether a file is text or binary:

1. **File Extension Analysis**: Yoink recognizes common text and binary file extensions, so it can quickly identify standard file types without analyzing content.

2. **MIME Type Detection**: For files with ambiguous or unknown extensions, Yoink uses signature-based detection to identify binary formats like images, videos, audio, archives, and more.

3. **Content Analysis**: As a final method, Yoink analyzes file content to determine if it's text by looking for patterns characteristic of text files:
   - Checks for null bytes (rare in text files)
   - Examines a larger sample (up to 4KB) of the file
   - Uses dynamic thresholds based on file size

This multi-layered approach makes Yoink much more accurate at handling various file types, avoiding errors when processing binary files, and ensuring you get clean text output.

### Parallel File Processing

Yoink leverages multi-core processing to analyze and process files in parallel:

1. **Efficient Resource Usage**: Automatically utilizes available CPU cores to process multiple files simultaneously.
   
2. **Performance Benefits**: Particularly noticeable when processing large directories with many files:
   - Up to 4-10x faster on systems with multiple cores
   - Scales with the number of available CPU cores
   
3. **Progress Tracking**: Shows a real-time progress bar with:
   - Visual indication of completion percentage
   - Estimated time remaining
   - Current file count
   
This parallel approach makes Yoink significantly more responsive when working with large codebases or directories containing many files.

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/yourusername/yoink.git
cd yoink

# Build and install
cargo install --path .
```

### Clipboard Manager Requirements

Yoink requires a clipboard manager to function properly. The application will automatically detect and use the appropriate clipboard manager for your system:

- **Linux (X11)**: xclip or xsel
  - Arch: `sudo pacman -S xclip`
  - Debian/Ubuntu: `sudo apt install xclip`
  - Fedora: `sudo dnf install xclip`

- **Linux (Wayland)**: wl-clipboard
  - Arch: `sudo pacman -S wl-clipboard`
  - Debian/Ubuntu: `sudo apt install wl-clipboard`
  - Fedora: `sudo dnf install wl-clipboard`

- **macOS**: pbcopy (pre-installed)

- **Android/Termux**: termux-api
  - `pkg install termux-api`

If you're using a specific desktop environment, Yoink also supports specialized clipboard managers for KDE, GNOME, Cinnamon, XFCE, MATE, and LXDE/LXQt.

## Usage

```bash
# Basic usage (copies all text files in current directory)
yoink

# Specify a directory or file
yoink /path/to/directory
yoink /path/to/file.txt

# Include only specific file extensions
yoink --extensions "rs,md,txt"

# Exclude specific file extensions
yoink --exclude "log,tmp,bak"

# Search for text within files
yoink --search "function main"

# Case-sensitive search
yoink --search "Function main" --case-sensitive

# Limit directory depth
yoink --depth 2

# Skip hidden files and directories
yoink --no-hidden

# Sort files by name
yoink --sort

# Save your configuration for future use
yoink --extensions "rs,md" --no-hidden --sort --save-config

# Ignore saved configuration
yoink --no-config
```

## Command-line Options

```
USAGE:
    yoink [OPTIONS] [PATH]

ARGS:
    <PATH>    Directory or file to yoink [default: .]

OPTIONS:
    -m, --max-size <SIZE>             Maximum file size in MB to consider [default: 10]
    -v, --verbose                     Show verbose output
    -d, --depth <DEPTH>               Maximum directory depth to traverse
    -e, --extensions <EXTS>           File extensions to include (comma-separated, e.g., "txt,md,rs")
    -x, --exclude <EXTS>              File extensions to exclude (comma-separated)
    --exclude-paths <PATHS>           Paths to exclude (comma-separated, exact names, not patterns)
    -p, --pattern <PATTERN>           Search pattern for filenames (supports glob patterns like *.txt)
    -H, --no-hidden                   Skip hidden files and directories
    -s, --sort                        Sort files by name before processing
    -S, --search <TEXT>               Search for text content within files
    -c, --case-sensitive              Make text search case-sensitive
    --save-config                     Save current configuration as default
    --no-config                       Ignore saved configuration file
    -h, --help                        Print help information
    -V, --version                     Print version information
```

## Output Format

The output copied to your clipboard will have the following format:

```
=== DIRECTORY STRUCTURE ===
📁 root/
  📁 src/
    📄 main.rs
    📄 utils.rs
  📁 docs/
    📄 README.md

=== TEXT FILES ===

=== main.rs ===
fn main() {
    println!("Hello, world!");
}

=== utils.rs ===
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

=== SUMMARY ===
Text files processed: 2
Binary files skipped: 0
```

When using the search feature, the output will include context around matches:

```
=== MATCH IN: src/main.rs ===
1: fn main() {
2:     println!("Hello, world!");
3: }

=== MATCH IN: src/lib.rs ===
10: pub fn process() {
11:     // Main processing function
12:     println!("Processing...");
13: }
...

=== SUMMARY ===
Text files processed: 2
Binary files skipped: 0
```

## License

This project is licensed under the GNU General Public License v3.0 - see the LICENSE file for details.