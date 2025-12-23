use std::fs;
use std::io::{BufWriter, Write};

fn main() {

  // Collect command-line arguments
    let arguments: Vec<String> = std::env::args().collect();

   // If there are multiple arguments, list contents for each specified path
    if arguments.len() != 1 {
        for arg in arguments.iter().skip(1){
            println!("{}:", arg);
            list_contents(arg);
            println!("");
        }
        // If no arguments are provided, list contents of the current directory
    }else{
        list_contents(".");
        
    }
    
}

 /// Function to list contents of a directory
fn list_contents(path: &str) {

    // handle to standard output
    let stdout = std::io::stdout();

    // buffered writer which store all the contents before printing them to the console
    let mut handle = BufWriter::new(stdout.lock());

     match fs::read_dir(path) {

        //read_dir gives an iterator over the entries within the directory
        Ok(entries) => {

            //using for loop to iterate over each entry in the directory
            for entry in entries {
                match entry {
                    Ok(dir_entry) => {

                        // Get the file type (directory or file)
                        let file_type = dir_entry.file_type().unwrap();

                        //check if the entry is a directory or a file and write to the handle accordingly
                        if file_type.is_dir() {

                            // Append a '/' to directory names, to distinguish them from files
                            // .to_string_lossy() converts OsString to String for display
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