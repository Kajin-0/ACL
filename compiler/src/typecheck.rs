use crate::{ast::*, error::CompileError};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct TypedProgram {
    pub program: Program,
    pub effect: Effect,
    pub tools: HashMap<String, ToolSignature>,
    pub manifest: CapabilityManifest,
}

pub fn typecheck(program: Program) -> Result<TypedProgram, CompileError> {
    let mut env = HashMap::<String, Type>::new();
    let mut effect = Effect::Pure;
    let mut tools = HashMap::<String, ToolSignature>::new();
    let mut caps = HashSet::<String>::new();

    for stmt in &program.statements {
        if let Stmt::ToolDecl {
            name,
            input,
            output,
            cap,
        } = stmt
        {
            let input_map = input.iter().cloned().collect::<HashMap<_, _>>();
            let output_map = output.iter().cloned().collect::<HashMap<_, _>>();
            tools.insert(
                name.clone(),
                ToolSignature {
                    input: input_map,
                    output: output_map,
                    cap: cap.clone(),
                },
            );
        }
    }

    for stmt in &program.statements {
        match stmt {
            Stmt::ToolDecl { .. } => {}
            Stmt::Let { name, expr } => {
                env.insert(name.clone(), infer_expr(expr, &env)?);
            }
            Stmt::Print { expr } => {
                let _ = infer_expr(expr, &env)?;
            }
            Stmt::ToolCall {
                tool, input, cap, ..
            } => {
                let sig = tools.get(tool).ok_or_else(|| {
                    CompileError::Type(format!("unknown tool declaration: {tool}"))
                })?;
                if sig.cap != *cap {
                    return Err(CompileError::Type(format!(
                        "tool {tool} requires cap {}, got {cap}",
                        sig.cap
                    )));
                }
                for (field, expected) in &sig.input {
                    let (_, expr) = input.iter().find(|(k, _)| k == field).ok_or_else(|| {
                        CompileError::Type(format!("missing required tool field: {field}"))
                    })?;
                    let actual = infer_expr(expr, &env)?;
                    if &actual != expected {
                        return Err(CompileError::Type(format!(
                            "tool field {field} expected {:?}, got {:?}",
                            expected, actual
                        )));
                    }
                }
                effect = Effect::Tool;
                caps.insert(cap.clone());
            }
        }
    }

    let mut required_caps = caps.into_iter().collect::<Vec<_>>();
    required_caps.sort();

    Ok(TypedProgram {
        program,
        effect,
        tools,
        manifest: CapabilityManifest { required_caps },
    })
}

fn infer_expr(expr: &Expr, env: &HashMap<String, Type>) -> Result<Type, CompileError> {
    match expr {
        Expr::Int(_) => Ok(Type::Int),
        Expr::Bool(_) => Ok(Type::Bool),
        Expr::String(_) => Ok(Type::String),
        Expr::Var(v) => env
            .get(v)
            .copied()
            .ok_or_else(|| CompileError::Type(format!("unknown variable: {v}"))),
        Expr::Binary { lhs, rhs, .. } => {
            let l = infer_expr(lhs, env)?;
            let r = infer_expr(rhs, env)?;
            if l == Type::Int && r == Type::Int {
                Ok(Type::Int)
            } else {
                Err(CompileError::Type(
                    "binary operations currently require Int operands".into(),
                ))
            }
        }
    }
}
