// Errors used across the codebase

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RuleEngineError {
    #[error("File not found: {0}")]
    FileNotFound(String),
}
