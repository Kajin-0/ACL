use axiom_compiler::{manifest::render_manifest, parser::parse_program, typecheck::typecheck};
use axiom_runtime::{
    replay::ReplayLog,
    runtime::{execute_with_defaults, ExecOptions},
    tools::ToolRegistry,
};
use std::{env, fs, path::PathBuf};

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(help());
    }
    match args[1].as_str() {
        "run" => {
            let file = args.get(2).ok_or_else(help)?;
            let replay_out = args
                .windows(2)
                .find(|w| w[0] == "--replay-out")
                .map(|w| PathBuf::from(&w[1]));
            let src = fs::read_to_string(file).map_err(|e| e.to_string())?;
            let typed = typecheck(parse_program(&src).map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())?;
            let log = execute_with_defaults(
                typed,
                &ToolRegistry::with_mock_tools(),
                ExecOptions::default(),
            )
            .map_err(|e| e.to_string())?;
            let digest = log.digest_hex();
            println!("replay_hash={digest}");
            if let Some(path) = replay_out {
                fs::write(path, log.to_text()).map_err(|e| e.to_string())?;
            }
        }
        "manifest" => {
            let file = args.get(2).ok_or_else(help)?;
            let src = fs::read_to_string(file).map_err(|e| e.to_string())?;
            let typed = typecheck(parse_program(&src).map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())?;
            print!("{}", render_manifest(&typed));
        }
        "replay-check" => {
            let file = args.get(2).ok_or_else(help)?;
            let text = fs::read_to_string(file).map_err(|e| e.to_string())?;
            let log = ReplayLog::from_text(&text)?;
            println!("replay_hash={}", log.digest_hex());
        }
        "fmt" => {
            let file = args.get(2).ok_or_else(help)?;
            let src = fs::read_to_string(file).map_err(|e| e.to_string())?;
            let out = src.lines().map(str::trim).collect::<Vec<_>>().join("\n") + "\n";
            fs::write(file, out).map_err(|e| e.to_string())?;
        }
        "lint" => {
            let file = args.get(2).ok_or_else(help)?;
            let src = fs::read_to_string(file).map_err(|e| e.to_string())?;
            let typed = typecheck(parse_program(&src).map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())?;
            if src.contains(" timeout 0;") {
                return Err("tool calls must use timeout > 0".to_string());
            }
            if typed.effect == axiom_compiler::Effect::Tool
                && typed.manifest.required_caps.is_empty()
            {
                return Err("tool effects require manifest capabilities".to_string());
            }
            println!("lint ok");
        }
        "test" => println!("use `cargo test --workspace`"),
        "bench" => println!("use parser bench tests in compiler/tests/parser_bench.rs"),
        "pkg" => {
            if args.iter().any(|a| a == "--lock") {
                fs::write("axiom.lock", "version = 1\nchecksum_algo = \"fnv1a64\"\n")
                    .map_err(|e| e.to_string())?;
                println!("wrote axiom.lock");
            } else {
                println!("axiom pkg --lock");
            }
        }
        _ => return Err(help()),
    }
    Ok(())
}

fn help() -> String {
    "usage: axiom <run|manifest|replay-check|fmt|lint|test|bench|pkg> ...".to_string()
}
