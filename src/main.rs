use std::env;
use std::fs;
use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    // Get current directory or specified directory
    let args: Vec<String> = env::args().collect();
    let path = if args.len() > 1 {
        &args[1]
    } else {
        "."
    };

    // Read and list the directory contents
    let entries = fs::read_dir(Path::new(path))?;

    for entry in entries {
        let entry = entry?;
        println!("{}", entry.file_name().into_string().unwrap());
    }

    Ok(())
}
