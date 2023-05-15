use std::env;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

fn read_files_with_extension(dir_path: &str, extension: &str) -> io::Result<Vec<String>> {
    let absolute_path = Path::new(dir_path).canonicalize()?;
    let files = fs::read_dir(absolute_path)?
        .filter_map(|entry| {
            if let Ok(entry) = entry {
                let file_path = entry.path();
                if file_path.is_file()
                    && file_path.extension().map(|ext| ext == extension) == Some(true)
                {
                    Some(file_path)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let mut file_contents = Vec::new();
    for file_path in files {
        let mut file = fs::File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        file_contents.push(contents);
    }

    Ok(file_contents)
}

pub fn get_all_solidity_files(directory: &str) -> Vec<String> {
    let current_dir = env::current_dir().expect("Failed to get the current directory");
    let full_path = current_dir.join(directory);

    match read_files_with_extension(full_path.to_str().unwrap(), "sol") {
        Ok(file_contents) => file_contents,
        Err(error) => {
            eprintln!("Error: {}", error);
            Vec::new() // Return an empty vector in case of error
        }
    }
}
