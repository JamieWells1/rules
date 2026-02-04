use crate::api::write::rule::write_with_base_dir;
use crate::err::RulesError;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

const TEST_CONFIG_DIR: &str = "src/api/tests/test_config";

fn create_test_tags() -> HashMap<String, Vec<String>> {
    let mut tags = HashMap::new();
    tags.insert(
        "colour".to_string(),
        vec!["red".to_string(), "blue".to_string(), "green".to_string()],
    );
    tags.insert(
        "size".to_string(),
        vec![
            "small".to_string(),
            "medium".to_string(),
            "large".to_string(),
        ],
    );
    tags
}

fn setup_and_cleanup_test_file(file_name: &str) {
    let _ = fs::create_dir_all(TEST_CONFIG_DIR);

    let path = format!("{}/{}", TEST_CONFIG_DIR, file_name);
    if Path::new(&path).exists() {
        let _ = fs::remove_file(&path);
    }
}

fn cleanup_test_file(file_name: &str) {
    let path = format!("{}/{}", TEST_CONFIG_DIR, file_name);
    if Path::new(&path).exists() {
        let _ = fs::remove_file(&path);
    }
}

#[test]
fn test_write_rule_creates_file() {
    let file_name = "test_create.rules";
    setup_and_cleanup_test_file(file_name);

    let tags = create_test_tags();
    let result = write_with_base_dir(file_name, "-colour = red", tags, TEST_CONFIG_DIR);

    assert!(result.is_ok());
    assert!(Path::new(&format!("{}/{}", TEST_CONFIG_DIR, file_name)).exists());

    cleanup_test_file(file_name);
}

#[test]
fn test_write_rule_normalises_filename() {
    let file_name_without_ext = "test_normalise";
    let file_name_with_ext = "test_normalise.rules";
    setup_and_cleanup_test_file(file_name_with_ext);

    let tags = create_test_tags();
    let result = write_with_base_dir(
        file_name_without_ext,
        "-colour = red",
        tags,
        TEST_CONFIG_DIR,
    );

    assert!(result.is_ok());
    assert!(Path::new(&format!("{}/{}", TEST_CONFIG_DIR, file_name_with_ext)).exists());

    cleanup_test_file(file_name_with_ext);
}

#[test]
fn test_write_rule_validates_rule() {
    let file_name = "test_validate.rules";
    setup_and_cleanup_test_file(file_name);

    let tags = create_test_tags();

    let result = write_with_base_dir(file_name, "colour = red", tags.clone(), TEST_CONFIG_DIR);
    assert!(result.is_err());

    let result = write_with_base_dir(
        file_name,
        "-invalid_tag = value",
        tags.clone(),
        TEST_CONFIG_DIR,
    );
    assert!(result.is_err());

    let result = write_with_base_dir(file_name, "-colour = purple", tags.clone(), TEST_CONFIG_DIR);
    assert!(result.is_err());

    cleanup_test_file(file_name);
}

#[test]
fn test_write_rule_appends_to_existing_file() {
    let file_name = "test_append.rules";
    setup_and_cleanup_test_file(file_name);

    let tags = create_test_tags();

    write_with_base_dir(file_name, "-colour = red", tags.clone(), TEST_CONFIG_DIR).unwrap();

    write_with_base_dir(file_name, "-size = large", tags.clone(), TEST_CONFIG_DIR).unwrap();

    let content = fs::read_to_string(&format!("{}/{}", TEST_CONFIG_DIR, file_name)).unwrap();
    assert!(content.contains("-colour = red"));
    assert!(content.contains("-size = large"));

    cleanup_test_file(file_name);
}

#[test]
fn test_write_rule_prevents_duplicates() {
    let file_name = "test_duplicate.rules";
    setup_and_cleanup_test_file(file_name);

    let tags = create_test_tags();

    write_with_base_dir(file_name, "-colour = red", tags.clone(), TEST_CONFIG_DIR).unwrap();

    let result = write_with_base_dir(file_name, "-colour = red", tags.clone(), TEST_CONFIG_DIR);
    assert!(result.is_err());
    if let Err(RulesError::RuleParseError(msg)) = result {
        assert!(msg.contains("already exists"));
    } else {
        panic!("Expected RuleParseError about duplicate");
    }

    cleanup_test_file(file_name);
}

#[test]
fn test_write_rule_creates_config_dir() {
    let test_dir = "src/api/tests/test_config_creation";

    if Path::new(test_dir).exists() {
        let _ = fs::remove_dir_all(test_dir);
    }

    let file_name = "test_dir.rules";
    let tags = create_test_tags();

    let result = write_with_base_dir(file_name, "-colour = red", tags, test_dir);
    assert!(result.is_ok());
    assert!(Path::new(test_dir).exists());

    let _ = fs::remove_dir_all(test_dir);
}

#[test]
fn test_write_rule_complex_rules() {
    let file_name = "test_complex.rules";
    setup_and_cleanup_test_file(file_name);

    let tags = create_test_tags();

    let complex_rules = vec![
        "-colour = red & size = large",
        "-colour = blue | size = small",
        "-(colour = red) & (size = large)",
    ];

    for rule in complex_rules {
        let result = write_with_base_dir(file_name, rule, tags.clone(), TEST_CONFIG_DIR);
        assert!(result.is_ok(), "Failed to write rule: {}", rule);
    }

    cleanup_test_file(file_name);
}

#[test]
fn test_write_rule_comma_syntax() {
    let file_name = "test_comma.rules";
    setup_and_cleanup_test_file(file_name);

    let tags = create_test_tags();

    let valid_comma_rules = vec![
        "-colour = red, blue",
        "-colour = red, blue, green",
        "-(colour = red, blue)",
        "-(colour = red, blue) & size = large",
        "-colour = red, blue & size = small",
        "-size = small, medium, large",
    ];

    for rule in valid_comma_rules {
        let result = write_with_base_dir(file_name, rule, tags.clone(), TEST_CONFIG_DIR);
        assert!(result.is_ok(), "Failed to write valid comma rule: {}", rule);
    }

    cleanup_test_file(file_name);
}

#[test]
fn test_write_rule_invalid_comma_syntax() {
    let file_name = "test_invalid_comma.rules";
    setup_and_cleanup_test_file(file_name);

    let tags = create_test_tags();

    let invalid_comma_rules = vec![
        "-colour =, red",
        "-,colour = red",
        "-colour = red,",
        "-(colour = red,) & size = large",
    ];

    for rule in invalid_comma_rules {
        let result = write_with_base_dir(file_name, rule, tags.clone(), TEST_CONFIG_DIR);
        assert!(
            result.is_err(),
            "Should reject invalid comma rule: {}",
            rule
        );
    }

    cleanup_test_file(file_name);
}

#[test]
fn test_write_rule_comma_with_different_operators() {
    let file_name = "test_comma_operators.rules";
    setup_and_cleanup_test_file(file_name);

    let tags = create_test_tags();

    let rules_with_different_ops = vec![
        "-colour = red, blue",
        "-colour ! red, blue",
        "-size ! small, medium",
    ];

    for rule in rules_with_different_ops {
        let result = write_with_base_dir(file_name, rule, tags.clone(), TEST_CONFIG_DIR);
        assert!(result.is_ok(), "Failed to write rule with comma: {}", rule);
    }

    cleanup_test_file(file_name);
}
