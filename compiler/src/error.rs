use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompileError {
    Parse(String),
    Type(String),
}

impl Display for CompileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CompileError::Parse(msg) => write!(f, "parse error: {msg}"),
            CompileError::Type(msg) => write!(f, "type error: {msg}"),
        }
    }
}
