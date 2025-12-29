use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use clap::Parser;
use colored::Colorize;
use std::time::SystemTime;

#[cfg(target_os = "windows")]
use std::os::windows::fs::MetadataExt;

#[cfg(target_os = "unix")]
use std::os::unix::fs::PermissionsExt;

use walkdir::{self, WalkDir};

#[derive(Parser)]
struct Arg {
    /// Paths of directories to list
    paths: Vec<String>,

    #[arg(short, long)]
    /// Show all files including hidden files
    all: bool,

    #[arg(short = 'R', long)]
    /// List directories recursively
    recursive: bool,

    #[arg(short = 't', long)]
    /// Sort files by modification time
    sort_by_time: bool,

    #[arg(short = 'r', long)]
    /// Reverse the order of the sort
    reverse: bool,

    #[arg(short = 'S', long)]
    /// sort by size
    sort_by_size: bool,

    #[arg(short = 'l', long)]
    /// Long format listing
    long_format: bool,

    #[arg(short = 'H', long)]
    /// Human-readable sizes
    human_readable: bool,
}

fn main() -> Result<()> {
    // Parse command-line arguments
    let arg = Arg::parse();

    // Collect the provided paths into a vector
    let paths: &[String] = &arg.paths;

    let separator = if arg.long_format { "\n" } else { " " };

    // If there are multiple arguments, list contents for each specified path
    if !paths.is_empty() {
        for path in paths.iter() {
            println!("{}:", path.green());
            let entries = collect_entries(path, &arg)
                .with_context(|| format!("Failed to read directory: {}", path))?; // Collect entries for the given path
            let display_entries = should_display(entries, &arg); // filter entries based on visibility
            let sorted_entries = sort_entries(display_entries, &arg); // sort entries based on criteria
            let formatted_entries = format_entries(sorted_entries, &arg); // format entries for display
            println!("{}", formatted_entries.join(separator)); // Print formatted entries
            println!(); // Print a newline for separation between different paths
        }
        // If no arguments are provided, list contents of the current directory
    } else {
        let entries = collect_entries(".", &arg).context("failed to read current directory")?;
        let display_entries = should_display(entries, &arg);
        let sorted_entries = sort_entries(display_entries, &arg);
        let formatted_entries = format_entries(sorted_entries, &arg);
        println!("{}", formatted_entries.join(separator));
    }
    Ok(())
}

// Function to collect entries from a directory based on the provided path and arguments(like recursive)
fn collect_entries(path: &str, arg: &Arg) -> Result<Vec<Entry>> {
    let mut results = Vec::new();

    // walker = interator over directory entries recursively or non-recursively based on arg.recursive
    let walker = if arg.recursive {
        WalkDir::new(path).min_depth(1)
    } else {
        WalkDir::new(path).max_depth(1).min_depth(1)
    };

    for entry in walker {
        match entry {
            Ok(dir_entry) => {
                let meta_data = dir_entry.metadata().with_context(|| {
                    format!("Failed to read metadata for {}", dir_entry.path().display())
                })?;

                let attribute: u32;

                #[cfg(target_os = "unix")]
                {
                    attribute = meta_data.permissions().mode();
                }
                #[cfg(target_os = "windows")]
                {
                    attribute = meta_data.file_attributes();
                }
                #[cfg(not(any(target_os = "unix", target_os = "windows")))]
                {
                    attribute = 0;
                }

                let entry_data = Entry {
                    name: if dir_entry.file_type().is_dir() {
                        format!("{}/", dir_entry.file_name().to_string_lossy())
                    } else {
                        format!("{}", dir_entry.file_name().to_string_lossy())
                    },
                    modified: meta_data.modified().with_context(|| {
                        format!(
                            "Failed to get modified time for {}",
                            dir_entry.path().display()
                        )
                    })?,
                    size: meta_data.len(),
                    attribute,
                };

                results.push(entry_data);
            }
            Err(e) => {
                eprintln!("Warning: {}", e);
            }
        }
    }

    Ok(results)
}

// Function to filter entries based on visibility (hidden or not)
fn should_display(entries: Vec<Entry>, arg: &Arg) -> Vec<Entry> {
    if arg.all {
        entries
    } else {
        entries
            .into_iter()
            .filter(|entry| {
                // Filter dot files on all platforms
                let is_dot_file = entry.name.starts_with(".");

                #[cfg(target_os = "windows")]
                let is_hidden = entry.attribute & 0x2 != 0; // Check HIDDEN attribute

                #[cfg(not(target_os = "windows"))]
                let is_hidden = false; // No additional hidden check on Unix

                !is_dot_file && !is_hidden
            })
            .collect()
    }
}

// Function to sort entries based on the provided arguments
fn sort_entries(mut entries: Vec<Entry>, arg: &Arg) -> Vec<Entry> {
    if arg.sort_by_time {
        entries.sort_by(|a, b| a.modified.cmp(&b.modified));
        if !arg.reverse {
            entries.reverse();
        }
    } else if arg.sort_by_size {
        entries.sort_by(|a, b| a.size.cmp(&b.size));
        if !arg.reverse {
            entries.reverse();
        }
    } else {
        // Default: sort alphabetically (case-insensitive)
        entries.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        if arg.reverse {
            entries.reverse();
        }
    }
    entries
}

// Function to format entries for display based on long_format and human_readable options
fn format_entries(entries: Vec<Entry>, arg: &Arg) -> Vec<String> {
    // taking each entry from the Vector and formatting it based on the long_format flag and human-readable size option
    let formatted_entries = entries
        .into_iter()
        .map(|f| {
            if arg.long_format {
                let datetime: DateTime<Local> = f.modified.into();
                let size_display = if arg.human_readable {
                    format_size(f.size)
                } else {
                    format!("{}B", f.size)
                };
                let attributes = parse_attributes(f.attribute);
                format!(
                    "{:<20}  {:>10} size  modified: {:<15} attributes: {}",
                    f.name,
                    size_display,
                    datetime.format("%b %d %H:%M"),
                    attributes
                )
            } else {
                f.name.to_string()
            }
        })
        .collect();

    formatted_entries
}

// Function to format file sizes into human-readable strings
fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.1}G", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1}M", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1}K", bytes as f64 / KB as f64)
    } else {
        format!("{}B", bytes)
    }
}
fn parse_attributes(attr: u32) -> String {
    #[cfg(target_os = "windows")]
    {
        let mut attributes = Vec::new();

        if attr & 0x1 != 0 {
            attributes.push("READONLY");
        }
        if attr & 0x2 != 0 {
            attributes.push("HIDDEN");
        }
        if attr & 0x4 != 0 {
            attributes.push("SYSTEM");
        }
        if attr & 0x20 != 0 {
            attributes.push("ARCHIVE");
        }

        if attributes.is_empty() {
            String::from("NORMAL")
        } else {
            attributes.join(", ")
        }
    }

    #[cfg(target_os = "unix")]
    {
        // Unix permissions (mode) - show as octal (e.g., 644, 755)
        format!("{:o}", attr & 0o777)
    }

    #[cfg(not(any(target_os = "unix", target_os = "windows")))]
    {
        String::from("UNKNOWN")
    }
}

// Struct to hold file entry information
#[derive(Debug)]
struct Entry {
    name: String,
    modified: SystemTime,
    size: u64,
    attribute: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(500), "500B");
        assert_eq!(format_size(2048), "2.0K");
        assert_eq!(format_size(5 * 1024 * 1024), "5.0M");
        assert_eq!(format_size(3 * 1024 * 1024 * 1024), "3.0G");
    }

    #[test]
    fn test_sort_by_name() {
        let entries = vec![
            Entry {
                name: "zebra".to_string(),
                modified: SystemTime::now(),
                size: 100,
                attribute: 0,
            },
            Entry {
                name: "apple".to_string(),
                modified: SystemTime::now(),
                size: 200,
                attribute: 0,
            },
        ];
        let arg = Arg {
            paths: vec![],
            all: false,
            recursive: false,
            sort_by_time: false,
            reverse: false,
            sort_by_size: false,
            long_format: false,
            human_readable: false,
        };
        let sorted = sort_entries(entries, &arg);
        assert_eq!(sorted[0].name, "apple");
        assert_eq!(sorted[1].name, "zebra");
    }

    #[test]
    fn test_sort_by_size() {
        let entries = vec![
            Entry {
                name: "small".to_string(),
                modified: SystemTime::now(),
                size: 100,
                attribute: 0,
            },
            Entry {
                name: "large".to_string(),
                modified: SystemTime::now(),
                size: 1000,
                attribute: 0,
            },
        ];
        let arg = Arg {
            paths: vec![],
            all: false,
            recursive: false,
            sort_by_time: false,
            reverse: false,
            sort_by_size: true,
            long_format: false,
            human_readable: false,
        };
        let sorted = sort_entries(entries, &arg);
        assert_eq!(sorted[0].name, "large"); // Largest first
        assert_eq!(sorted[1].name, "small");
    }

    #[test]
    fn test_reverse_sort() {
        let entries = vec![
            Entry {
                name: "a".to_string(),
                modified: SystemTime::now(),
                size: 100,
                attribute: 0,
            },
            Entry {
                name: "z".to_string(),
                modified: SystemTime::now(),
                size: 200,
                attribute: 0,
            },
        ];
        let arg = Arg {
            paths: vec![],
            all: false,
            recursive: false,
            sort_by_time: false,
            reverse: true,
            sort_by_size: false,
            long_format: false,
            human_readable: false,
        };
        let sorted = sort_entries(entries, &arg);
        assert_eq!(sorted[0].name, "z");
        assert_eq!(sorted[1].name, "a");
    }

    #[test]
    fn test_should_display_filters_hidden() {
        let entries = vec![
            Entry {
                name: ".hidden".to_string(),
                modified: SystemTime::now(),
                size: 100,
                attribute: 0,
            },
            Entry {
                name: "visible".to_string(),
                modified: SystemTime::now(),
                size: 200,
                attribute: 0,
            },
        ];
        let arg = Arg {
            paths: vec![],
            all: false,
            recursive: false,
            sort_by_time: false,
            reverse: false,
            sort_by_size: false,
            long_format: false,
            human_readable: false,
        };
        let filtered = should_display(entries, &arg);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "visible");
    }

    #[test]
    fn test_should_display_shows_all() {
        let entries = vec![
            Entry {
                name: ".hidden".to_string(),
                modified: SystemTime::now(),
                size: 100,
                attribute: 0,
            },
            Entry {
                name: "visible".to_string(),
                modified: SystemTime::now(),
                size: 200,
                attribute: 0,
            },
        ];
        let arg = Arg {
            paths: vec![],
            all: true,
            recursive: false,
            sort_by_time: false,
            reverse: false,
            sort_by_size: false,
            long_format: false,
            human_readable: false,
        };
        let filtered = should_display(entries, &arg);
        assert_eq!(filtered.len(), 2);
    }

    #[test]
    fn test_format_entries_short() {
        let entries = vec![Entry {
            name: "test.txt".to_string(),
            modified: SystemTime::now(),
            size: 1024,
            attribute: 0,
        }];
        let arg = Arg {
            paths: vec![],
            all: false,
            recursive: false,
            sort_by_time: false,
            reverse: false,
            sort_by_size: false,
            long_format: false,
            human_readable: false,
        };
        let formatted = format_entries(entries, &arg);
        assert_eq!(formatted[0], "test.txt");
    }

    #[test]
    fn test_format_entries_with_human_readable() {
        let entries = vec![Entry {
            name: "test.txt".to_string(),
            modified: SystemTime::now(),
            size: 2048,
            attribute: 0,
        }];
        let arg = Arg {
            paths: vec![],
            all: false,
            recursive: false,
            sort_by_time: false,
            reverse: false,
            sort_by_size: false,
            long_format: true,
            human_readable: true,
        };
        let formatted = format_entries(entries, &arg);
        assert!(formatted[0].contains("2.0K"));
    }
}
