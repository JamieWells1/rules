// File utils

use std::fs;

fn read_file(path: &str) -> Result<String, std::io::error> {
    let contents: String = fs::read_to_string(path)?;
    Ok(contents)
}

fn read_files_in_dir(dir_path: &str) -> Result<Vec<String, std::io::error>> {
    let mut contents = Vec::new();

    for file in glob("config/*.rules")? {
        let path = entry?;
        contents.push(read_file(path)?);
    }

    Ok(contents)
}
