use solang_parser::pt::SourceUnit;
use std::env;
use std::fs;
use std::io::{self, Read};
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct FileNameWithContent {
    pub file_path: String,
    pub filename: String,
    pub file_content: String,
    pub parsed_ast_tree: SourceUnit,
}

pub fn get_all_solidity_files(directory: &str) -> Vec<FileNameWithContent> {
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

fn read_files_with_extension(
    dir_path: &str,
    extension: &str,
) -> io::Result<Vec<FileNameWithContent>> {
    let absolute_path = Path::new(dir_path).canonicalize()?;
    let files = WalkDir::new(absolute_path)
        .into_iter()
        .filter_map(|entry| {
            if let Ok(entry) = entry {
                let file_path = entry.path();
                if file_path.is_file()
                    && file_path.extension().map(|ext| ext == extension) == Some(true)
                {
                    Some(file_path.to_owned())
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
        // Skip paths that contain "node_modules"
        if file_path.to_string_lossy().contains("node_modules")
            || file_path.to_string_lossy().contains("lib")
            || file_path.to_string_lossy().contains("test")
            || file_path.to_string_lossy().contains("script")
        {
            continue;
        }

        let mut file = fs::File::open(&file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let filename = file_path.file_name().unwrap().to_string_lossy().to_string();
        let mut file_relative_path = file_path
            .strip_prefix(dir_path)
            .unwrap()
            .to_string_lossy()
            .to_string();
        file_relative_path.insert_str(0, "./");

        file_contents.push(FileNameWithContent {
            filename,
            file_path: file_relative_path,
            file_content: contents,
            parsed_ast_tree: SourceUnit(vec![]),
        });
    }

    Ok(file_contents)
}
