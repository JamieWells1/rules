// Types

// Aliases

use std::collections::HashMap;

// colour
pub type TagName = String;

// red, green
pub type TagValues = Vec<String>;

// 1
pub type SubRuleNumber = i32;

// "colour": ["green"]
pub type Object = HashMap<String, Vec<String>>;

// Structs

pub struct Tag {
    pub name: TagName,
    pub values: TagValues,
}

pub enum ComparisonOp {
    ISEQ,
    NOEQ,
    // To be supported in future:
    // GREQ,
    // LEEQ,
}

pub enum LogicalOp {
    AND,
    OR,
}

pub struct SubRule {
    pub expected_count: i32,
    pub actual_count: i32,
    pub comparison_ops: Vec<ComparisonOp>,
}

// Impls

impl Default for SubRule {
    fn default() -> Self {
        SubRule {
            expected_count: 2,
            actual_count: 0,
            comparison_ops: vec![],
        }
    }
}
