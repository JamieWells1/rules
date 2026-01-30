// Types

// Aliases

pub type TagName = String;
pub type TagValues = Vec<String>;
pub type SubRuleNumber = i32;

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
