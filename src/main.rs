use std::{fmt::format, time::SystemTime};

use chrono::{DateTime, Local};
use clap::Parser;
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
}

fn main() {
    // Parse command-line arguments
    let arg = Arg::parse();

    // Collect the provided paths into a vector
    let paths: &[String] = &arg.paths;

    let seperator = if arg.long_format { "\n" } else { " " };

    // If there are multiple arguments, list contents for each specified path
    if !paths.is_empty() {
        for path in paths.iter() {
            println!("{}:", path);
            let entries = collect_entries(path, &arg); // Collect entries for the given path
            let display_entries = should_display(entries, &arg); // filter entries based on visibility
            let sorted_entries = sort_entries(display_entries, &arg); // sort entries based on criteria
            let formatted_entries = format_entries(sorted_entries, &arg); // format entries for display
            println!("{}", formatted_entries.join(seperator)); // Print formatted entries
            println!(""); // Print a newline for separation between different paths
        }
        // If no arguments are provided, list contents of the current directory
    } else {
        let entries = collect_entries(".", &arg); 
        let display_entries = should_display(entries, &arg);
        let sorted_entries = sort_entries(display_entries, &arg);
        let formatted_entries = format_entries(sorted_entries, &arg);
        println!("{}", formatted_entries.join(seperator));
    }
}

fn collect_entries(path: &str, arg: &Arg) -> Vec<Entry> {
    let mut results = Vec::new();

    let walker = if arg.recursive {
        WalkDir::new(path).min_depth(1)
    } else {
        WalkDir::new(path).max_depth(1).min_depth(1)
    };

    for entry in walker {
        match entry {
            Ok(dir_entry) => {
                let entry_data = Entry {
                    name: if dir_entry.file_type().is_dir() {
                        format!("{}/", dir_entry.file_name().to_string_lossy())
                    } else {
                        format!("{}", dir_entry.file_name().to_string_lossy())
                    },
                    modified: dir_entry
                        .metadata()
                        .unwrap()
                        .modified()
                        .unwrap_or(SystemTime::now()),
                    size: dir_entry.metadata().unwrap().len(),
                };

                results.push(entry_data);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }

    results
}

fn should_display(entries: Vec<Entry>, arg: &Arg) -> Vec<Entry> {
    let mut result = Vec::new();

    if arg.all {
        result = entries
    } else if !arg.all {
        result = entries
            .into_iter()
            .filter(|entry| !entry.name.starts_with("."))
            .collect();
    }

    result
}

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
    }
    entries
}

fn format_entries(entries: Vec<Entry>, arg: &Arg) -> Vec<String> {
    let formated_entries = entries
        .into_iter()
        .map(|f| {
            if arg.long_format {
                let datetime: DateTime<Local> = f.modified.into();
                format!(
                    "{:<20}  {:>10} bytes  modified: {:<15}",
                    f.name,
                    f.size,
                    datetime.format("%b %d %H:%M")
                )
            } else {
                format!("{}", f.name)
            }
        })
        .collect();

    formated_entries
}

// Struct to hold file entry information
struct Entry {
    name: String,
    modified: std::time::SystemTime,
    size: u64,
}
