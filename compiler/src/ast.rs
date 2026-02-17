use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    ToolDecl {
        name: String,
        input: Vec<(String, Type)>,
        output: Vec<(String, Type)>,
        cap: String,
    },
    Let {
        name: String,
        expr: Expr,
    },
    Print {
        expr: Expr,
    },
    ToolCall {
        tool: String,
        input: Vec<(String, Expr)>,
        cap: String,
        timeout_ms: u64,
    },
}

#[derive(Debug, Clone)]
pub enum Expr {
    Int(i64),
    Bool(bool),
    String(String),
    Var(String),
    Binary {
        lhs: Box<Expr>,
        op: BinOp,
        rhs: Box<Expr>,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Int,
    Bool,
    String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Effect {
    Pure,
    Tool,
}

#[derive(Debug, Clone)]
pub struct ToolSignature {
    pub input: HashMap<String, Type>,
    pub output: HashMap<String, Type>,
    pub cap: String,
}

#[derive(Debug, Clone)]
pub struct CapabilityManifest {
    pub required_caps: Vec<String>,
}
