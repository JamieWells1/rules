use crate::Rules;
use std::fs;
use std::path::Path;

const TEST_CONFIG_DIR: &str = "src/api/tests/test_config";

fn setup_test_tags(test_name: &str) -> String {
    let _ = fs::create_dir_all(TEST_CONFIG_DIR);

    let tags_file = format!("{}/{}.tags", TEST_CONFIG_DIR, test_name);
    let tags_content = "# Test tags\n- colour: red, blue, green\n- shape: circle, square, rectangle\n- size: small, medium, large";
    fs::write(&tags_file, tags_content).unwrap();
    tags_file
}

fn cleanup_test_file(file_name: &str) {
    let path = format!("{}/{}", TEST_CONFIG_DIR, file_name);
    if Path::new(&path).exists() {
        let _ = fs::remove_file(&path);
    }
}

#[test]
fn test_rules_api_load_and_validate() {
    let _tags_file = setup_test_tags("test_load_validate");

    let mut rules = Rules::new(TEST_CONFIG_DIR);
    rules.load_tags().unwrap();

    assert!(rules.validate_rule("- colour = red").is_ok());
    assert!(rules.validate_rule("- shape = circle").is_ok());
    assert!(rules.validate_rule("- size = large").is_ok());

    assert!(
        rules
            .validate_rule("- colour = red & shape ! circle")
            .is_ok()
    );
    assert!(
        rules
            .validate_rule("- colour = blue | size = small")
            .is_ok()
    );

    assert!(rules.validate_rule("- colour = red, blue").is_ok());
    assert!(rules.validate_rule("- colour = red, blue, green").is_ok());

    assert!(
        rules
            .validate_rule("- (colour = red, blue) & size = large")
            .is_ok()
    );

    assert!(rules.validate_rule("- invalid = value").is_err());

    assert!(rules.validate_rule("- colour = purple").is_err());

    cleanup_test_file("test_load_validate.tags");
}

#[test]
fn test_rules_api_case_insensitive_loading() {
    let _tags_file = setup_test_tags("test_case_insensitive");

    let mut rules = Rules::new(TEST_CONFIG_DIR);
    rules.load_tags().unwrap();

    rules.debug_tags();

    assert!(rules.validate_rule("- colour = red").is_ok());
    assert!(rules.validate_rule("- colour = blue").is_ok());
    assert!(rules.validate_rule("- shape = circle").is_ok());

    cleanup_test_file("test_case_insensitive.tags");
}

#[test]
fn test_rules_api_write_methods() {
    let _tags_file = setup_test_tags("test_write_methods");

    let mut rules = Rules::new(TEST_CONFIG_DIR);
    rules.load_tags().unwrap();

    rules
        .write_tag("api_test", "material", vec!["wood", "metal", "plastic"])
        .unwrap();

    assert!(rules.validate_rule("- material = wood").is_ok());

    rules
        .write_rule("api_test", "- colour = red & size = large")
        .unwrap();

    cleanup_test_file("test_write_methods.tags");
    cleanup_test_file("api_test.tags");
    cleanup_test_file("api_test.rules");
}
