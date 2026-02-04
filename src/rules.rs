use crate::err::RulesError;
use crate::parser::rules::RuleParser;
use crate::parser::tags;
use crate::types::{TagName, TagValues};
use crate::utils::file;
use std::collections::HashMap;

/// Main API for the rules engine.
///
/// Provides methods for managing tags, rules, objects, and evaluating rules
/// against objects.
///
/// # Examples
/// ```ignore
/// use rules::Rules;
///
/// // Create a new Rules instance with a config directory
/// let mut rules = Rules::new("config");
///
/// // Add tags
/// rules.write_tag("colours.tags", "colour", vec!["red", "blue"])?;
///
/// // Add rules that reference those tags
/// rules.write_rule("my_rules.rules", "- colour = red")?;
///
/// // Validate a rule
/// rules.validate_rule("- colour = blue")?;
/// ```
pub struct Rules {
    /// Base directory for config files
    config_dir: String,
    /// Cached tags loaded from config files
    tags: HashMap<TagName, TagValues>,
}

impl Rules {
    /// Creates a new Rules instance with the specified config directory.
    ///
    /// # Arguments
    /// * `config_dir` - Path to the directory containing .tags, .rules, and .yaml files
    ///
    /// # Examples
    /// ```ignore
    /// let rules = Rules::new("config");
    /// ```
    pub fn new(config_dir: impl Into<String>) -> Self {
        Self {
            config_dir: config_dir.into(),
            tags: HashMap::new(),
        }
    }

    /// Loads all tags from .tags files in the config directory.
    ///
    /// This should be called after creating a new Rules instance to populate
    /// the tag definitions needed for rule validation.
    ///
    /// # Returns
    /// * `Ok(())` if tags were loaded successfully
    /// * `Err(RulesError)` if loading fails
    ///
    /// # Examples
    /// ```ignore
    /// let mut rules = Rules::new("config");
    /// rules.load_tags()?;
    /// ```
    pub fn load_tags(&mut self) -> Result<(), RulesError> {
        let pattern = format!("{}/*.tags", self.config_dir);
        let all_files = file::read_files_in_dir(&pattern)?;

        // Clear existing tags
        self.tags.clear();

        for file_content in all_files.iter() {
            for line in file_content.lines() {
                if file::line_blank_or_comment(line) {
                    continue;
                }

                let (name, values) = tags::get_name_and_values_from_tag(line)?;

                // Normalize to lowercase for consistent lookup
                let name = name.to_lowercase();
                let values: Vec<String> = values.iter().map(|v| v.to_lowercase()).collect();

                // Merge values if tag already exists
                self.tags
                    .entry(name)
                    .and_modify(|existing_values| existing_values.extend(values.clone()))
                    .or_insert(values);
            }
        }

        Ok(())
    }

    /// Writes a tag to a .tags file.
    ///
    /// # Arguments
    /// * `file_name` - Name of the file (with or without .tags extension)
    /// * `tag_name` - Name of the tag (without the leading '-')
    /// * `tag_values` - Vector of values for the tag
    ///
    /// # Examples
    /// ```ignore
    /// rules.write_tag("my_tags", "colour", vec!["red", "blue"])?;
    /// ```
    pub fn write_tag(
        &mut self,
        file_name: &str,
        tag_name: impl Into<String>,
        tag_values: Vec<impl Into<String>>,
    ) -> Result<(), RulesError> {
        let tag_name = tag_name.into();
        let tag_values: Vec<String> = tag_values.into_iter().map(|v| v.into()).collect();

        // Write to file
        crate::api::write::tag::write_with_base_dir(
            file_name,
            tag_name.clone(),
            tag_values.clone(),
            &self.config_dir,
        )?;

        // Normalize to lowercase for consistent lookup in cache
        let tag_name_lower = tag_name.to_lowercase();
        let tag_values_lower: Vec<String> = tag_values.iter().map(|v| v.to_lowercase()).collect();

        // Update cached tags (append if exists)
        self.tags
            .entry(tag_name_lower)
            .and_modify(|existing| existing.extend(tag_values_lower.clone()))
            .or_insert(tag_values_lower);

        Ok(())
    }

    /// Writes a rule to a .rules file.
    ///
    /// The rule is validated against the current tag definitions before writing.
    ///
    /// # Arguments
    /// * `file_name` - Name of the file (with or without .rules extension)
    /// * `rule` - The rule string to write (should start with '-')
    ///
    /// # Examples
    /// ```ignore
    /// rules.write_rule("my_rules", "- colour = red & size = large")?;
    /// ```
    pub fn write_rule(&self, file_name: &str, rule: &str) -> Result<(), RulesError> {
        crate::api::write::rule::write_with_base_dir(
            file_name,
            rule,
            self.tags.clone(),
            &self.config_dir,
        )
    }

    /// Writes an object definition to a .yaml file.
    ///
    /// # Arguments
    /// * `file_name` - Name of the file
    /// * `obj_type` - Type/category of the object
    /// * `obj` - HashMap representing the object's properties
    ///
    /// # Examples
    /// ```ignore
    /// let mut obj = HashMap::new();
    /// obj.insert("colour".to_string(), vec!["red".to_string()]);
    /// rules.write_object("objects.yaml", "shapes", obj)?;
    /// ```
    pub fn write_object(
        &self,
        file_name: &str,
        obj_type: impl Into<String>,
        obj: HashMap<String, Vec<String>>,
    ) -> Result<(), RulesError> {
        crate::api::write::object::write_with_base_dir(
            file_name,
            obj_type.into(),
            obj,
            &self.config_dir,
        )
    }

    /// Validates a rule string against the current tag definitions.
    ///
    /// This checks syntax and ensures all referenced tags and values exist.
    ///
    /// # Arguments
    /// * `rule` - The rule string to validate (should start with '-')
    ///
    /// # Returns
    /// * `Ok(())` if the rule is valid
    /// * `Err(RulesError)` with details if validation fails
    ///
    /// # Examples
    /// ```ignore
    /// rules.validate_rule("- colour = red & size = large")?;
    /// ```
    pub fn validate_rule(&self, rule: &str) -> Result<(), RulesError> {
        let parser = RuleParser::new(self.tags.clone());
        parser.validate_rule(rule)
    }

    /// Evaluates rules against objects.
    ///
    /// Note: Currently uses the default orchestrator which reads from the config directory.
    /// Parameters will be supported in a future update.
    ///
    /// # Examples
    /// ```ignore
    /// rules.evaluate()?;
    /// ```
    pub fn evaluate(&self) -> Result<(), RulesError> {
        crate::api::entry::evaluate()
    }

    /// Debug method to print loaded tags
    #[cfg(test)]
    pub fn debug_tags(&self) {
        println!("Loaded tags:");
        for (name, values) in &self.tags {
            println!("  '{}' -> {:?}", name, values);
        }
    }
}
