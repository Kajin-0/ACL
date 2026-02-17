use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeError {
    MissingCapability(String),
    InvalidCapability(String),
    InvalidTimeout(String),
    MissingToolSignature(String),
    ToolValidation(String),
    ToolExecution(String),
    Eval(String),
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::MissingCapability(s) => write!(f, "missing capability: {s}"),
            RuntimeError::InvalidCapability(s) => write!(f, "invalid capability: {s}"),
            RuntimeError::InvalidTimeout(s) => write!(f, "invalid timeout: {s}"),
            RuntimeError::MissingToolSignature(s) => write!(f, "missing tool signature: {s}"),
            RuntimeError::ToolValidation(s) => write!(f, "tool validation failed: {s}"),
            RuntimeError::ToolExecution(s) => write!(f, "tool execution failed: {s}"),
            RuntimeError::Eval(s) => write!(f, "evaluation error: {s}"),
        }
    }
}
