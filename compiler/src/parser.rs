use crate::{ast::*, error::CompileError};

pub fn parse_program(src: &str) -> Result<Program, CompileError> {
    let mut statements = Vec::new();
    for (line_no, raw) in src.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with("//") {
            continue;
        }
        if let Some(rest) = line.strip_prefix("tool ") {
            // tool Name input {a: Int} output {b: String} cap toolCap;
            let (name, tail) = rest.split_once(" input ").ok_or_else(|| {
                CompileError::Parse(format!("line {}: invalid tool decl", line_no + 1))
            })?;
            let (input_raw, tail) = parse_braced_section(tail, line_no, "input")?;
            let tail = tail.strip_prefix("output ").ok_or_else(|| {
                CompileError::Parse(format!("line {}: expected output", line_no + 1))
            })?;
            let (output_raw, tail) = parse_braced_section(tail, line_no, "output")?;
            let tail = tail
                .trim()
                .strip_prefix("cap ")
                .ok_or_else(|| CompileError::Parse(format!("line {}: expected cap", line_no + 1)))?
                .trim_end_matches(';')
                .trim();
            statements.push(Stmt::ToolDecl {
                name: name.trim().to_string(),
                input: parse_typed_fields(input_raw, line_no)?,
                output: parse_typed_fields(output_raw, line_no)?,
                cap: tail.to_string(),
            });
            continue;
        }
        if let Some(rest) = line.strip_prefix("let ") {
            let (name, expr_part) = rest
                .split_once('=')
                .ok_or_else(|| CompileError::Parse(format!("line {}: invalid let", line_no + 1)))?;
            let expr = parse_expr(expr_part.trim().trim_end_matches(';'))?;
            statements.push(Stmt::Let {
                name: name.trim().to_string(),
                expr,
            });
            continue;
        }
        if let Some(rest) = line.strip_prefix("print ") {
            statements.push(Stmt::Print {
                expr: parse_expr(rest.trim().trim_end_matches(';'))?,
            });
            continue;
        }
        if let Some(rest) = line.strip_prefix("call ") {
            let (tool, tail) = rest.split_once('{').ok_or_else(|| {
                CompileError::Parse(format!("line {}: missing '{{'", line_no + 1))
            })?;
            let (input_raw, tail) = tail.split_once('}').ok_or_else(|| {
                CompileError::Parse(format!("line {}: missing '}}'", line_no + 1))
            })?;
            let mut input = Vec::new();
            for part in input_raw
                .split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
            {
                let (k, v) = part.split_once(':').ok_or_else(|| {
                    CompileError::Parse(format!("line {}: invalid tool input", line_no + 1))
                })?;
                input.push((k.trim().to_string(), parse_expr(v.trim())?));
            }
            let tail = tail.trim().trim_end_matches(';');
            let tail = tail.strip_prefix("using ").ok_or_else(|| {
                CompileError::Parse(format!("line {}: expected using", line_no + 1))
            })?;
            let (cap, timeout_part) = tail.split_once(" timeout ").ok_or_else(|| {
                CompileError::Parse(format!("line {}: expected timeout", line_no + 1))
            })?;
            let timeout_ms = timeout_part
                .trim()
                .parse::<u64>()
                .map_err(|e| CompileError::Parse(e.to_string()))?;
            statements.push(Stmt::ToolCall {
                tool: tool.trim().to_string(),
                input,
                cap: cap.trim().to_string(),
                timeout_ms,
            });
            continue;
        }
        return Err(CompileError::Parse(format!(
            "line {}: unrecognized statement",
            line_no + 1
        )));
    }
    Ok(Program { statements })
}

fn parse_braced_section<'a>(
    tail: &'a str,
    line_no: usize,
    _name: &str,
) -> Result<(&'a str, &'a str), CompileError> {
    let tail = tail.trim();
    let tail = tail
        .strip_prefix('{')
        .ok_or_else(|| CompileError::Parse(format!("line {}: expected '{{'", line_no + 1)))?;
    let (inside, rest) = tail
        .split_once('}')
        .ok_or_else(|| CompileError::Parse(format!("line {}: missing '}}'", line_no + 1)))?;
    Ok((inside, rest.trim()))
}

fn parse_typed_fields(raw: &str, line_no: usize) -> Result<Vec<(String, Type)>, CompileError> {
    let mut out = Vec::new();
    for part in raw.split(',').map(str::trim).filter(|s| !s.is_empty()) {
        let (k, t) = part.split_once(':').ok_or_else(|| {
            CompileError::Parse(format!("line {}: invalid typed field", line_no + 1))
        })?;
        out.push((k.trim().to_string(), parse_type(t.trim())?));
    }
    Ok(out)
}

fn parse_type(raw: &str) -> Result<Type, CompileError> {
    match raw {
        "Int" => Ok(Type::Int),
        "Bool" => Ok(Type::Bool),
        "String" => Ok(Type::String),
        _ => Err(CompileError::Parse(format!("unknown type: {raw}"))),
    }
}

fn parse_expr(raw: &str) -> Result<Expr, CompileError> {
    let raw = raw.trim();
    if raw.starts_with('"') && raw.ends_with('"') {
        return Ok(Expr::String(raw.trim_matches('"').to_string()));
    }
    if raw == "true" {
        return Ok(Expr::Bool(true));
    }
    if raw == "false" {
        return Ok(Expr::Bool(false));
    }
    if let Ok(v) = raw.parse::<i64>() {
        return Ok(Expr::Int(v));
    }
    for (c, op) in [
        ('+', BinOp::Add),
        ('-', BinOp::Sub),
        ('*', BinOp::Mul),
        ('/', BinOp::Div),
    ] {
        if let Some((lhs, rhs)) = split_once_top_level(raw, c) {
            return Ok(Expr::Binary {
                lhs: Box::new(parse_expr(lhs)?),
                op,
                rhs: Box::new(parse_expr(rhs)?),
            });
        }
    }
    Ok(Expr::Var(raw.to_string()))
}

fn split_once_top_level(s: &str, needle: char) -> Option<(&str, &str)> {
    let mut in_string = false;
    for (idx, ch) in s.char_indices() {
        match ch {
            '"' => in_string = !in_string,
            _ if !in_string && ch == needle => return Some((&s[..idx], &s[idx + 1..])),
            _ => {}
        }
    }
    None
}
