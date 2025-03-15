use std::process::{Command, Stdio};
use std::io::Write;
use std::thread;
use std::time::Duration;
use indicatif::{ProgressBar, ProgressStyle};
use colored::*;
use super::ClipboardInterface;

pub struct ClipboardManager {
    verbose: bool,
}

impl ClipboardManager {
    pub fn new(verbose: bool) -> Self {
        Self { verbose }
    }

    fn try_copy_to_clipboard(&self, text: &str) -> Result<(), String> {
        // Check for macOS
        let macos_methods = [
            (vec!["pbcopy"], "pbcopy (macOS)"),
        ];
        if self.try_methods(&macos_methods, text)? {
            return Ok(());
        }

        // Check for Android/Termux
        let termux_methods = [
            (vec!["termux-clipboard-set"], "termux-clipboard-set (Android/Termux)"),
        ];
        if self.try_methods(&termux_methods, text)? {
            return Ok(());
        }

        // Check for Linux/X11
        let linux_methods = [
            (vec!["xclip", "-selection", "clipboard"], "xclip (Linux/X11)"),
            (vec!["xsel", "-b"], "xsel (Linux/X11)"),
            (vec!["wl-copy"], "wl-copy (Wayland)"),
        ];
        if self.try_methods(&linux_methods, text)? {
            return Ok(());
        }

        Err("No clipboard utility found. Please make sure you have one of the following installed: xclip, xsel (Linux/X11), wl-copy (Wayland), pbcopy (macOS), or termux-clipboard-set (Android/Termux)".to_string())
    }

    fn try_methods(&self, methods: &[(Vec<&str>, &str)], text: &str) -> Result<bool, String> {
        for (cmd_args, name) in methods {
            if let Some(cmd) = cmd_args.first() {
                match Command::new(cmd)
                    .args(&cmd_args[1..])
                    .stdin(Stdio::piped())
                    .stderr(Stdio::null())
                    .stdout(Stdio::null())
                    .spawn() {
                    Ok(mut child) => {
                        if let Some(mut stdin) = child.stdin.take() {
                            match stdin.write_all(text.as_bytes()) {
                                Ok(_) => {
                                    drop(stdin);
                                    match child.wait() {
                                        Ok(exit) => {
                                            if exit.success() {
                                                if self.verbose {
                                                    println!("Text copied using {}", name);
                                                }
                                                return Ok(true);
                                            }
                                        }
                                        Err(e) => {
                                            if self.verbose {
                                                println!("Error waiting for clipboard process to finish: {}", e);
                                            }
                                        }
                                    }
                                }
                                Err(e) => {
                                    if self.verbose {
                                        println!("Error writing to clipboard: {}", e);
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        if self.verbose {
                            println!("Command '{}' not available: {}", cmd, e);
                        }
                    }
                }
            }
        }
        
        Ok(false)
    }
}

impl ClipboardInterface for ClipboardManager {
    fn copy_to_clipboard(&self, text: &str) -> Result<(), String> {
        // Show a progress spinner for clipboard operations
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
                .template("{spinner:.green} {msg}")
                .unwrap()
        );
        pb.set_message("Copying to clipboard...");
        pb.enable_steady_tick(Duration::from_millis(80));

        let result = self.try_copy_to_clipboard(text);
        
        // Finish the progress bar
        pb.finish_and_clear();
        
        result
    }
} 