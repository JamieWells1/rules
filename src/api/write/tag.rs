use crate::err::RulesError;
use crate::parser::tags;
use std::fs;
use std::path::Path;

fn normalise_filename(file_name: &str) -> String {
    if file_name.ends_with(".tags") {
        file_name.to_string()
    } else {
        format!("{}.tags", file_name)
    }
}

fn ensure_config_dir(base_dir: &str) -> Result<(), RulesError> {
    let config_dir = Path::new(base_dir);
    if !config_dir.exists() {
        fs::create_dir_all(config_dir)?;
    }
    Ok(())
}

pub fn write(file_name: &str, tag_name: String, tag_values: Vec<String>) -> Result<(), RulesError> {
    write_with_base_dir(file_name, tag_name, tag_values, "config")
}

#[cfg(test)]
pub(crate) fn write_with_base_dir(
    file_name: &str,
    tag_name: String,
    tag_values: Vec<String>,
    base_dir: &str,
) -> Result<(), RulesError> {
    write_internal(file_name, tag_name, tag_values, base_dir)
}

#[cfg(not(test))]
pub(crate) fn write_with_base_dir(
    file_name: &str,
    tag_name: String,
    tag_values: Vec<String>,
    base_dir: &str,
) -> Result<(), RulesError> {
    write_internal(file_name, tag_name, tag_values, base_dir)
}

fn write_internal(
    file_name: &str,
    tag_name: String,
    tag_values: Vec<String>,
    base_dir: &str,
) -> Result<(), RulesError> {
    if tag_name.trim().is_empty() {
        return Err(RulesError::TagParseError(
            "Tag name cannot be empty".to_string(),
        ));
    }

    if tag_values.is_empty() {
        return Err(RulesError::TagParseError(
            "Tag must have at least one value".to_string(),
        ));
    }

    if tag_name.contains(' ') {
        return Err(RulesError::TagParseError(
            "Tag name cannot contain spaces".to_string(),
        ));
    }

    for value in &tag_values {
        if value.contains(' ') {
            return Err(RulesError::TagParseError(format!(
                "Tag value '{}' cannot contain spaces",
                value
            )));
        }
    }

    let normalised_name = normalise_filename(file_name);
    let full_path = format!("{}/{}", base_dir, normalised_name);

    ensure_config_dir(base_dir)?;

    let mut lines: Vec<String> = if Path::new(&full_path).exists() {
        fs::read_to_string(&full_path)?
            .lines()
            .map(|l: &str| l.to_string())
            .collect()
    } else {
        Vec::new()
    };

    let mut tag_exists = false;
    let tag_name_trimmed = tag_name.trim().to_lowercase();

    for line in &mut lines {
        if line.trim().starts_with('#') || line.trim().is_empty() {
            continue;
        }

        match tags::get_name_and_values_from_tag(line) {
            Ok((extracted_name, _extracted_values)) => {
                if extracted_name.trim() == tag_name_trimmed {
                    line.push_str(&format!(", {}", tag_values.join(", ")));
                    tag_exists = true;
                    break;
                }
            }
            Err(_) => {
                continue;
            }
        }
    }

    if !tag_exists {
        let new_tag = format!("- {}: {}", tag_name_trimmed, tag_values.join(", "));
        lines.push(new_tag);
    }

    fs::write(&full_path, lines.join("\n"))?;

    Ok(())
}
