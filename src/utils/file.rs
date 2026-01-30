// File utils
use crate::err::RulesError;

use std::fs;

use glob::glob;

pub fn read_files_in_dir(pattern: &str) -> Result<Vec<String>, RulesError> {
    let mut contents = Vec::new();

    for entry in glob(pattern)? {
        let path = entry?;
        let file_content = fs::read_to_string(path)?;
        contents.push(file_content);
    }

    Ok(contents)
}
