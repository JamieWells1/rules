// Parser for .tags files
use crate::err::RulesError;
use crate::utils::file;

pub struct Tag {
    pub name: String,
    pub values: Vec<String>,
}

pub fn parse_tags() -> Result<Vec<Tag>, RulesError> {
    let all_files: Vec<String> = file::read_files_in_dir("config/*.tags")?;

    for file in all_files.iter() {
        println!("{}", file);
    }
    Ok(vec![])
}
