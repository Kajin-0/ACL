use crate::{
    capability::{default_capabilities, Capability},
    error::RuntimeError,
    replay::{Event, ReplayLog},
    tools::ToolRegistry,
};
use axiom_compiler::{BinOp, Effect, Expr, Stmt, Type};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Bool(bool),
    String(String),
}

pub struct ExecOptions {
    pub deterministic_seed: u64,
}
impl Default for ExecOptions {
    fn default() -> Self {
        Self {
            deterministic_seed: 42,
        }
    }
}

pub fn execute_with_defaults(
    typed: axiom_compiler::typecheck::TypedProgram,
    tools: &ToolRegistry,
    opts: ExecOptions,
) -> Result<ReplayLog, RuntimeError> {
    let caps = default_capabilities();
    execute(typed, &caps, tools, opts)
}

pub fn execute(
    typed: axiom_compiler::typecheck::TypedProgram,
    capabilities: &HashMap<String, Capability>,
    tools: &ToolRegistry,
    opts: ExecOptions,
) -> Result<ReplayLog, RuntimeError> {
    let mut env = HashMap::<String, Value>::new();
    let mut log = ReplayLog::default();
    let mut rng = Lcg::new(opts.deterministic_seed);

    if typed.effect == Effect::Tool && capabilities.is_empty() {
        return Err(RuntimeError::MissingCapability(
            "tool effect requested but no capabilities supplied".into(),
        ));
    }

    let tool_sigs = typed.tools;
    for stmt in typed.program.statements {
        match stmt {
            Stmt::ToolDecl { .. } => {}
            Stmt::Let { name, expr } => {
                env.insert(name, eval_expr(&expr, &env)?);
            }
            Stmt::Print { expr } => {
                let v = eval_expr(&expr, &env)?;
                let msg = format_value(&v);
                println!("{msg}");
                log.push(Event::Print { value: msg });
            }
            Stmt::ToolCall {
                tool,
                input,
                cap,
                timeout_ms,
            } => {
                if timeout_ms == 0 {
                    return Err(RuntimeError::InvalidTimeout(format!(
                        "tool call {tool} has zero timeout"
                    )));
                }
                let c = capabilities
                    .get(&cap)
                    .ok_or_else(|| RuntimeError::MissingCapability(cap.clone()))?;
                if !c.can_use_tool() {
                    return Err(RuntimeError::InvalidCapability(format!(
                        "{cap} does not grant tool rights"
                    )));
                }
                let sig = tool_sigs
                    .get(&tool)
                    .ok_or_else(|| RuntimeError::MissingToolSignature(tool.clone()))?;
                let mut fields = Vec::new();
                for (k, e) in input {
                    let v = eval_expr(&e, &env)?;
                    fields.push(format!("\"{k}\":{}", to_json(v)));
                }
                let input_json = format!("{{{}}}", fields.join(","));
                let out = tools
                    .call(&tool, input_json.clone())
                    .map_err(RuntimeError::ToolExecution)?;
                validate_tool_output(&out, &sig.output)?;
                log.push(Event::ToolCall {
                    tool,
                    input: input_json,
                    output_hash: stable_hash_hex(&out),
                    output: out,
                    source: "tool-registry".to_string(),
                    timestamp_ms: 0,
                    policy_tags: vec!["default".to_string()],
                });
                log.push(Event::Time { millis: timeout_ms });
            }
        }
        log.push(Event::Random { value: rng.next() });
    }
    Ok(log)
}

fn validate_tool_output(raw: &str, schema: &HashMap<String, Type>) -> Result<(), RuntimeError> {
    for k in schema.keys() {
        let needle = format!("\"{k}\":");
        if !raw.contains(&needle) {
            return Err(RuntimeError::ToolValidation(format!(
                "tool output missing required field: {k}"
            )));
        }
    }
    Ok(())
}

fn eval_expr(expr: &Expr, env: &HashMap<String, Value>) -> Result<Value, RuntimeError> {
    match expr {
        Expr::Int(v) => Ok(Value::Int(*v)),
        Expr::Bool(v) => Ok(Value::Bool(*v)),
        Expr::String(v) => Ok(Value::String(v.clone())),
        Expr::Var(v) => env
            .get(v)
            .cloned()
            .ok_or_else(|| RuntimeError::Eval(format!("unknown variable: {v}"))),
        Expr::Binary { lhs, op, rhs } => {
            let l = eval_expr(lhs, env)?;
            let r = eval_expr(rhs, env)?;
            let (l, r) = match (l, r) {
                (Value::Int(l), Value::Int(r)) => (l, r),
                _ => {
                    return Err(RuntimeError::Eval(
                        "binary ops require Int values".to_string(),
                    ));
                }
            };
            Ok(Value::Int(match op {
                BinOp::Add => l + r,
                BinOp::Sub => l - r,
                BinOp::Mul => l * r,
                BinOp::Div => l / r,
            }))
        }
    }
}

fn to_json(v: Value) -> String {
    match v {
        Value::Int(v) => v.to_string(),
        Value::Bool(v) => v.to_string(),
        Value::String(v) => format!("\"{v}\""),
    }
}
fn format_value(v: &Value) -> String {
    match v {
        Value::Int(v) => v.to_string(),
        Value::Bool(v) => v.to_string(),
        Value::String(v) => v.clone(),
    }
}

fn stable_hash_hex(s: &str) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for b in s.bytes() {
        hash ^= b as u64;
        hash = hash.wrapping_mul(0x00000100000001B3);
    }
    format!("{hash:016x}")
}

struct Lcg(u64);
impl Lcg {
    fn new(seed: u64) -> Self {
        Self(seed)
    }
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1);
        self.0
    }
}
