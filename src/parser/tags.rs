use std::collections::HashSet;

// Parser for .tags files
use crate::err::RulesError;
use crate::types::Tag;
use crate::utils::file;
use crate::utils::string::{StringUtils, normalise};

pub fn validate_tag(line: &str) -> Result<(), RulesError> {
    if file::line_blank_or_comment(line) {
        return Ok(());
    }

    let parts: Vec<&str> = line.split(":").collect();
    let mut errors: HashSet<&str> = HashSet::new();

    // Check parts length BEFORE accessing
    if parts.len() < 2 {
        return Err(RulesError::TagParseError(
            "Tag must contain a ':' separator".to_string(),
        ));
    }

    if parts.len() > 2 {
        errors.insert("Tag must only contain one name and one set of values. Ensure there is only one semi-colon");
    }

    // NOW safe to access parts[0] and parts[1]
    let name: &str = parts[0];
    let values: &str = parts[1];

    if let Some(first_char) = name.trim().at(0) {
        if first_char != '-' {
            errors.insert("Tag must begin with '-'");
        }

        let name_no_dash: String = name.trim().chars().skip(1).collect();
        if name_no_dash.split_whitespace().count() > 1 {
            errors.insert("Tag name cannot contain spaces");
        }
    }

    for value in values.split(",") {
        // Contains space and it isn't trailing or leading
        if value.trim().contains(" ") {
            errors.insert("Tag values cannot contain spaces");
        }
    }

    if !errors.is_empty() {
        // Copy references to strings from HashSet into Vec to join as one string
        let error_list: Vec<&str> = errors.iter().copied().collect();

        return Err(RulesError::TagParseError(format!(
            "Errors parsing line: '{}': {}",
            line,
            error_list.join(", ")
        )));
    }

    Ok(())
}

fn get_name_from_tag(parts: &Vec<&str>) -> Result<String, RulesError> {
    normalise(parts[0])
}

fn get_values_from_tag(parts: &Vec<&str>) -> Vec<String> {
    parts[1].split(',').map(|v| v.trim().to_string()).collect()
}

pub fn get_name_and_values_from_tag(line: &str) -> Result<(String, Vec<String>), RulesError> {
    validate_tag(line)?;
    let parts: Vec<&str> = line.trim().split(':').collect();

    let name: String = get_name_from_tag(&parts)?;
    let values: Vec<String> = get_values_from_tag(&parts);
    Ok((name, values))
}

pub fn parse_tags() -> Result<Vec<Tag>, RulesError> {
    let mut tags: Vec<Tag> = Vec::new();
    let all_files: Vec<String> = file::read_files_in_dir("config/*.tags")?;

    for file in all_files.iter() {
        for line in file.lines() {
            if file::line_blank_or_comment(line) {
                continue;
            }

            let raw_tag = get_name_and_values_from_tag(line)?;
            let tag = Tag {
                name: raw_tag.0,
                values: raw_tag.1,
            };
            tags.push(tag);
        }
    }

    Ok(tags)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_tag_valid() {
        let valid_tag = "- Color: Red, Blue, Green";
        assert!(validate_tag(valid_tag).is_ok());
    }

    #[test]
    fn test_validate_tag_skips_comments() {
        let comment = "# This is a comment";
        assert!(validate_tag(comment).is_ok());
    }

    #[test]
    fn test_validate_tag_skips_empty_lines() {
        let empty = "";
        assert!(validate_tag(empty).is_ok());
    }

    #[test]
    fn test_validate_tag_no_colon() {
        let invalid_tag = "- Color Red Blue";
        let result = validate_tag(invalid_tag);

        assert!(result.is_err());
        if let Err(RulesError::TagParseError(msg)) = result {
            assert!(msg.contains("must contain a ':' separator"));
        } else {
            panic!("Expected TagParseError about missing colon");
        }
    }

    #[test]
    fn test_validate_tag_no_dash() {
        let invalid_tag = "Color: Red, Blue";
        let result = validate_tag(invalid_tag);

        assert!(result.is_err());
        if let Err(RulesError::TagParseError(msg)) = result {
            assert!(msg.contains("Tag must begin with '-'"));
        } else {
            panic!("Expected TagParseError about missing dash");
        }
    }

    #[test]
    fn test_validate_tag_name_with_spaces() {
        let invalid_tag = "- Color Name: Red, Blue";
        let result = validate_tag(invalid_tag);

        assert!(result.is_err());
        if let Err(RulesError::TagParseError(msg)) = result {
            assert!(msg.contains("Tag name cannot contain spaces"));
        } else {
            panic!("Expected TagParseError about tag name spaces");
        }
    }

    #[test]
    fn test_validate_tag_value_with_middle_spaces() {
        let invalid_tag = "- Color: Dark Blue, Red";
        let result = validate_tag(invalid_tag);

        assert!(result.is_err());
        if let Err(RulesError::TagParseError(msg)) = result {
            assert!(msg.contains("Tag values cannot contain spaces"));
        } else {
            panic!("Expected TagParseError about value spaces");
        }
    }

    #[test]
    fn test_validate_tag_multiple_colons() {
        let invalid_tag = "- Color: Red: Blue";
        let result = validate_tag(invalid_tag);

        assert!(result.is_err());
        if let Err(RulesError::TagParseError(msg)) = result {
            assert!(msg.contains("only one") || msg.contains("semi-colon"));
        } else {
            panic!("Expected TagParseError about multiple colons");
        }
    }

    #[test]
    fn test_validate_tag_with_leading_whitespace() {
        let tag_with_whitespace = "  - Color: Red, Blue";
        assert!(validate_tag(tag_with_whitespace).is_ok());
    }

    #[test]
    fn test_get_name_and_values_from_tag() {
        let tag = "- Color: Red, Blue, Green";
        let result = get_name_and_values_from_tag(tag);

        assert!(result.is_ok());
        if let Ok((name, values)) = result {
            assert_eq!(name, "Color");
            assert_eq!(values.len(), 3);
            assert!(values.contains(&"Red".to_string()));
            assert!(values.contains(&"Blue".to_string()));
            assert!(values.contains(&"Green".to_string()));
        }
    }

    #[test]
    fn test_get_name_and_values_trims_whitespace() {
        let tag = "  - Color  :  Red ,  Blue  ";
        let result = get_name_and_values_from_tag(tag);

        assert!(result.is_ok());
        if let Ok((name, values)) = result {
            assert_eq!(name, "Color");
            assert_eq!(values, vec!["Red".to_string(), "Blue".to_string()]);
        }
    }
}
