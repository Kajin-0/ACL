use axiom_compiler::parser::parse_program;

#[test]
fn parser_smoke_bench() {
    let mut src = String::new();
    for i in 0..10_000 {
        src.push_str(&format!("let x{i} = {i};\n"));
    }
    let parsed = parse_program(&src).expect("parse");
    assert_eq!(parsed.statements.len(), 10_000);
}
