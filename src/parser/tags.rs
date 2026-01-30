// Parser for .tags files
use crate::err::RulesError;
use crate::types::Tag;
use crate::utils::file;

pub fn parse_tags() -> Result<Vec<Tag>, RulesError> {
    let all_files: Vec<String> = file::read_files_in_dir("config/*.tags")?;

    for file in all_files.iter() {
        println!("{}", file);
    }
    Ok(vec![])
}
