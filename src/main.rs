use std::fs;
use std::io::{BufWriter, Write};

fn main() {
    println!("Hello, world!");
    let arguments: Vec<String> = std::env::args().collect();

    let path = if arguments.len() == 1 {
        String::from(".")
    } else {
        arguments[1].clone()
    };
    
    let stdout = std::io::stdout();
    let mut handle = BufWriter::new(stdout.lock());

    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(dir_entry) => {
                        let file_type = dir_entry.file_type().unwrap();
                        if file_type.is_dir() {
                            writeln!(handle, "{}/", dir_entry.file_name().to_string_lossy())
                                .unwrap();
                        } else if file_type.is_file() {
                            
                            writeln!(handle, "{}", dir_entry.file_name().to_string_lossy()).unwrap();
                        }
                    }

                    Err(e) => {
                        writeln!(handle, "Error: {}", e).unwrap();
                    }
                }
            }
        }

        Err(e) => writeln!(handle, "Error: {}", e).unwrap(),
    }
}
