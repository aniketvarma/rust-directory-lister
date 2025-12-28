# VW - Directory Lister

A simple command-line tool to list files and directories, written in Rust! This is my first Rust project where I'm learning systems programming.

## What Does It Do?

It's basically like the `ls` command (or `dir` on Windows) but with some cool features I wanted to add while learning Rust. You can list files, sort them different ways, and see file information.

## Features

- 📁 List files in any directory
- 🔍 Show hidden files with `-a`
- 📂 List files recursively (go into subdirectories) with `-R`
- 📊 Sort by name, size, or modification time
- 🔄 Reverse the sort order
- 📝 Long format showing file sizes and dates
- 📏 Human-readable file sizes (like 2.5M instead of 2621440)

## Installation

First, make sure you have Rust installed. Then:

```bash
# Clone this repo
git clone https://github.com/aniketvarma/rust-directory-lister
cd rust-directory-lister

# Build the project
cargo build --release

# The binary will be at target/release/vw.exe
```

Or install it globally:
```bash
cargo install --path .
```

## How to Use

### Basic Usage

```bash
# List current directory
vw

# List a specific directory
vw src/

# List multiple directories
vw src/ target/
```

### Options

| Flag | What it does |
|------|-------------|
| `-a` or `--all` | Show hidden files (files starting with `.`) |
| `-R` or `--recursive` | List files in subdirectories too |
| `-l` or `--long-format` | Show detailed info (size, date modified) |
| `-H` or `--human-readable` | Show file sizes like "2.5M" instead of bytes |
| `-t` or `--sort-by-time` | Sort by when files were last modified |
| `-S` or `--sort-by-size` | Sort by file size (biggest first) |
| `-r` or `--reverse` | Reverse the sorting order |

### Examples

```bash
# Show all files including hidden ones
vw -a

# Long format with human-readable sizes
vw -l -H

# List everything recursively, sorted by size
vw -R -S

# Show recent files first
vw -t

# Show oldest files first
vw -t -r

# Detailed view of specific directory
vw src/ -l -H
```

## What I Learned

While building this, I learned:
- How to parse command-line arguments with `clap`
- Working with the file system using `walkdir`
- Formatting dates and times with `chrono`
- Error handling with `anyhow`
- Writing tests in Rust
- How to sort and filter data
- Working with structs and vectors

## Built With

- [clap](https://crates.io/crates/clap) - For handling command-line arguments
- [walkdir](https://crates.io/crates/walkdir) - For walking through directories
- [chrono](https://crates.io/crates/chrono) - For formatting dates
- [anyhow](https://crates.io/crates/anyhow) - For easier error handling

## Testing

Run the tests with:
```bash
cargo test
```

## Future Ideas

Things I might add later:
- File ownership display on Windows
- Color coding for different file types
- Filter by file extension
- Show file permissions

## Contributing

This is a learning project, but feel free to open issues or PRs if you have suggestions!

## License

Feel free to use this however you want!

---

Made while learning Rust 🦀
