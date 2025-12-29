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
- 🎨 Directories displayed in green
- 🔒 Shows Windows file attributes (READONLY, HIDDEN, SYSTEM, ARCHIVE) or Unix permissions
- 🌍 **Cross-platform** - Works on Windows, Linux, and macOS

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

# Detailed view of specific directory with attributes
vw src/ -l -H
```

### Sample Output

```
# Standard listing
Cargo.lock  Cargo.toml  README.md  src/  target/

# Long format (-l -H)
Cargo.lock                 11.7K size  modified: Dec 27 23:07    attributes: ARCHIVE
Cargo.toml                  235B size  modified: Dec 27 23:07    attributes: ARCHIVE
README.md                   3.2K size  modified: Dec 29 12:19    attributes: ARCHIVE
src/                          0B size  modified: Dec 27 17:34    attributes: NORMAL
target/                       0B size  modified: Dec 28 19:33    attributes: NORMAL
```
Directories are displayed in **green** for easy identification.

## What I Learned

While building this, I learned:
- How to parse command-line arguments with `clap`
- Working with the file system using `walkdir`
- Formatting dates and times with `chrono`
- Error handling with `anyhow`
- **Cross-platform development** - Using conditional compilation (`#[cfg]`) for Windows and Unix
- **Platform-specific APIs** - Windows file attributes vs Unix permissions
- Working with traits like `MetadataExt` for platform-specific features
- ANSI color codes and the `colored` crate
- Writing tests in Rust
- How to sort and filter data
- Working with structs and vectors

## Platform-Specific Features

### Windows
- Displays file attributes: READONLY, HIDDEN, SYSTEM, ARCHIVE
- Hides files that start with `.` (Unix convention) OR have the Windows HIDDEN attribute set
- Uses Windows Console API for colored output

### Unix/Linux/macOS  
- Displays file permissions in octal format (e.g., 644, 755)
- Hides files starting with `.` (Unix convention)
- Uses ANSI escape codes for colored output

## Built With

- [clap](https://crates.io/crates/clap) - For handling command-line arguments
- [walkdir](https://crates.io/crates/walkdir) - For walking through directories
- [chrono](https://crates.io/crates/chrono) - For formatting dates
- [anyhow](https://crates.io/crates/anyhow) - For easier error handling
- [colored](https://crates.io/crates/colored) - For cross-platform colored terminal output

## Testing

Run the tests with:
```bash
cargo test
```

## Future Ideas

Things I might add later:
- File ownership display (user/group info)
- More color coding for different file types (executables, archives, etc.)
- Filter by file extension
- Tree view for recursive listings
- Symlink detection and display

## Contributing

This is a learning project, but feel free to open issues or PRs if you have suggestions!

## License

Feel free to use this however you want!

---

Made while learning Rust 🦀
