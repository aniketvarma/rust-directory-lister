# VW - Directory Lister

 A command-line tool to list the files and directories. I wanted to learn Rust by rebuilding something familiar like `ls`, and it turned out to be way more complex than I expected!

## What Does It Do?

It's basically like the `ls` command (or `dir` on Windows). Started simple, then I kept adding features as I learned more about Rust - sorting, colors, cross-platform support, etc.

## Features

- 📁 List files in any directory
- 🔍 Show hidden files with `-a`
- 📂 List files recursively (go into subdirectories) with `-R`
- 📊 Sort by name, size, or modification time
- 🔄 Reverse the sort order
- 📝 Long format showing file sizes and dates
- 📏 Human-readable file sizes (like 2.5M instead of 2621440)
- 🎨 Directories displayed in green
- 🔒 Shows Windows file attributes (READONLY, HIDDEN, SYSTEM, ARCHIVE) or Unix permissions (only in long format)
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

## What I Learned (and Struggled With!)

Building this taught me a lot about Rust:

**Things that clicked:**
- `clap` made argument parsing surprisingly easy
- `walkdir` is awesome for traversing directories
- The borrow checker is annoying at first but actually catches real bugs
- `Result` and `?` for error handling makes sense once you get it

**Things that took forever to figure out:**
- **Conditional compilation** - Spent way too long figuring out that `#[cfg(target_os = "windows")]` only applies to the next line, not entire blocks. Had to wrap everything in braces.
- **Windows vs Unix differences** - Files starting with `.` aren't hidden on Windows by default! I assumed they were like Unix. Also, Windows uses "attributes" while Unix uses "permissions" - completely different systems.
- **The `attribute` variable scope issue** - Got compile errors because the variable wasn't initialized on all platforms. Had to add a fallback for non-Windows/Unix systems.
- **Making colored output work on Windows** - Turns out older Windows versions don't support ANSI colors. The `colored` crate handles this automatically, which is nice.
- **Trait imports** - Had to import `MetadataExt` even though I was calling methods on `Metadata`. Still wrapping my head around how traits work.

**Bugs I fixed:**
- Initially forgot to add "B" suffix when not using human-readable format
- Hidden file logic was backwards at first 
- Directory names weren't showing in color because I was checking the wrong condition

## Platform-Specific Stuff

One interesting thing I learned: Windows and Unix handle file visibility completely differently!

**Windows:**
- Shows file attributes (READONLY, HIDDEN, SYSTEM, ARCHIVE)
- I made it hide both `.` files AND files with the HIDDEN attribute to be consistent with Unix

**Unix/Linux/macOS:**
- Shows permissions in octal (644, 755, etc.)
- Only hides files starting with `.`

This was probably the most confusing part - getting cross-platform behavior right without breaking either platform.

## Built With

- [clap](https://crates.io/crates/clap) - Command-line argument parsing
- [walkdir](https://crates.io/crates/walkdir) - Directory traversal
- [chrono](https://crates.io/crates/chrono) - Date/time formatting
- [anyhow](https://crates.io/crates/anyhow) - Error handling (way better than writing custom error types)
- [colored](https://crates.io/crates/colored) - Terminal colors that actually work on Windows

## Known Issues / TODO

- [ ] Long format alignment breaks with really long filenames


## Testing

Run tests:
```bash
cargo test
```

## Future Ideas

Stuff I want to add when I have time:
- Tree view for recursive listings (this might be hard)
- Show total size/file count at the end
- File ownership info (need to figure out how to get user/group on both platforms)

## Contributing

Code probably isn't perfect. If you see something that could be improved or have suggestions, feel free to open an issue!

## License

MIT - do whatever you want with it

---
