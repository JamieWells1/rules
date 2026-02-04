use crate::err::RulesError;
use crate::parser::rules::RuleParser;
use crate::types::{TagName, TagValues};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn normalise_filename(file_name: &str) -> String {
    if file_name.ends_with(".rules") {
        file_name.to_string()
    } else {
        format!("{}.rules", file_name)
    }
}

fn ensure_config_dir(base_dir: &str) -> Result<(), RulesError> {
    let config_dir = Path::new(base_dir);
    if !config_dir.exists() {
        fs::create_dir_all(config_dir)?;
    }
    Ok(())
}

pub fn write(
    file_name: &str,
    rule: &str,
    tags: HashMap<TagName, TagValues>,
) -> Result<(), RulesError> {
    write_with_base_dir(file_name, rule, tags, "config")
}

#[cfg(test)]
pub(crate) fn write_with_base_dir(
    file_name: &str,
    rule: &str,
    tags: HashMap<TagName, TagValues>,
    base_dir: &str,
) -> Result<(), RulesError> {
    let base = base_dir;

    write_internal(file_name, rule, tags, base)
}

#[cfg(not(test))]
pub(crate) fn write_with_base_dir(
    file_name: &str,
    rule: &str,
    tags: HashMap<TagName, TagValues>,
    base_dir: &str,
) -> Result<(), RulesError> {
    write_internal(file_name, rule, tags, base_dir)
}

fn write_internal(
    file_name: &str,
    rule: &str,
    tags: HashMap<TagName, TagValues>,
    base_dir: &str,
) -> Result<(), RulesError> {
    // normalise filename
    let normalised_name = normalise_filename(file_name);
    let full_path = format!("{}/{}", base_dir, normalised_name);

    ensure_config_dir(base_dir)?;

    let parser = RuleParser::new(tags);
    parser.validate_rule(rule)?;

    // Read existing file or create new content
    let mut lines: Vec<String> = if Path::new(&full_path).exists() {
        fs::read_to_string(&full_path)?
            .lines()
            .map(|l: &str| l.to_string())
            .collect()
    } else {
        Vec::new()
    };

    let rule_trimmed = rule.trim();
    if lines
        .iter()
        .any(|line: &String| line.trim() == rule_trimmed)
    {
        return Err(RulesError::RuleParseError(
            "Rule already exists in file".to_string(),
        ));
    }

    lines.push(rule_trimmed.to_string());

    fs::write(&full_path, lines.join("\n"))?;

    Ok(())
}
