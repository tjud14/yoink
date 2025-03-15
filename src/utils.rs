use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

// General-purpose text file detection based on content analysis
pub fn is_text(data: &[u8]) -> bool {
    if data.is_empty() {
        return false;
    }

    // Use the infer crate to detect known binary file types
    if let Some(kind) = infer::get(data) {
        // If it's a known binary format (image, video, audio, archive, etc.), it's not text
        if is_binary_mime_type(kind.mime_type()) {
            return false;
        }
    }

    // Count null bytes - text files rarely have null bytes
    let null_byte_count = data.iter().take(4096).filter(|&&b| b == 0).count();
    if null_byte_count > 0 {
        return false;
    }

    // Examine a larger sample size (up to 4KB) for more accurate detection
    let sample_size = data.len().min(4096);
    let sample = &data[..sample_size];
    
    // Count printable characters and common control chars (newlines, tabs)
    let text_chars = sample.iter().filter(|&&b| {
        b >= 32 || b == b'\n' || b == b'\r' || b == b'\t'
    }).count();

    // Higher threshold for short files, lower for larger samples
    let threshold = if sample_size < 100 { 0.95 } else { 0.8 };
    
    (text_chars as f32 / sample_size as f32) >= threshold
}

// Load a file and determine if it's a text file
pub fn is_text_file(path: &Path) -> io::Result<bool> {
    // First check file extension for common text formats
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        let ext = ext.to_lowercase();
        if is_common_text_extension(&ext) {
            return Ok(true);
        }
        if is_common_binary_extension(&ext) {
            return Ok(false);
        }
    }

    // For other files, examine the content
    let mut file = File::open(path)?;
    
    // Read up to 8KB for analysis (sufficient for file type detection)
    let mut buffer = vec![0; 8192];
    let bytes_read = file.read(&mut buffer)?;
    buffer.truncate(bytes_read);
    
    Ok(is_text(&buffer))
}

// List of common text file extensions
fn is_common_text_extension(ext: &str) -> bool {
    matches!(ext, 
        "txt" | "md" | "markdown" | "rst" | 
        "html" | "htm" | "css" | "scss" | "sass" | "less" |
        "js" | "jsx" | "ts" | "tsx" | "json" | "toml" | "yaml" | "yml" |
        "rs" | "go" | "py" | "rb" | "php" | "java" | "kt" | "c" | "cpp" | "h" | "hpp" |
        "cs" | "fs" | "swift" | "scala" | "groovy" | "pl" | "sh" | "bash" | "zsh" |
        "ini" | "cfg" | "conf" | "config" | "properties" | "xml" | "svg" |
        "sql" | "graphql" | "gql" | "r" | "lua" | "ex" | "exs" | "elm" | "clj" |
        "hs" | "erl" | "lisp" | "dart"
    )
}

// List of common binary file extensions
fn is_common_binary_extension(ext: &str) -> bool {
    matches!(ext,
        "exe" | "dll" | "so" | "dylib" | "bin" | "o" | "a" | "lib" | "obj" |
        "png" | "jpg" | "jpeg" | "gif" | "bmp" | "tiff" | "webp" | "ico" | "heic" |
        "mp3" | "wav" | "ogg" | "flac" | "m4a" | "aac" | "wma" |
        "mp4" | "avi" | "mov" | "mkv" | "flv" | "wmv" | "webm" |
        "pdf" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" |
        "zip" | "rar" | "tar" | "gz" | "7z" | "bz2" | "xz" | "iso" |
        "db" | "sqlite" | "mdb" | "class" | "pyc" | "pyo" | "pyd" |
        "ttf" | "otf" | "woff" | "woff2" | "eot" |
        "dat" | "pb" | "deb" | "rpm" | "tgz" | "pkg"
    )
}

// Check if a MIME type is likely a binary format
fn is_binary_mime_type(mime_type: &str) -> bool {
    mime_type.starts_with("image/") || 
    mime_type.starts_with("video/") || 
    mime_type.starts_with("audio/") || 
    mime_type.starts_with("application/") && !mime_type.contains("json") && !mime_type.contains("xml") && !mime_type.contains("text")
}