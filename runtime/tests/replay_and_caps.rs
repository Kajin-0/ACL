use axiom_compiler::{parser::parse_program, typecheck::typecheck};
use axiom_runtime::{
    replay::ReplayLog,
    runtime::{execute_with_defaults, ExecOptions},
    tools::ToolRegistry,
};

#[test]
fn replay_hash_is_stable_for_same_program() {
    let src = "tool MockEcho input { value: Int } output { echo: String } cap toolCap;\nlet x = 1 + 2;\nprint x;\ncall MockEcho { value: x } using toolCap timeout 1000;\n";
    let typed_a = typecheck(parse_program(src).expect("parse")).expect("typecheck");
    let typed_b = typecheck(parse_program(src).expect("parse")).expect("typecheck");

    let a = execute_with_defaults(
        typed_a,
        &ToolRegistry::with_mock_tools(),
        ExecOptions::default(),
    )
    .expect("execute");
    let b = execute_with_defaults(
        typed_b,
        &ToolRegistry::with_mock_tools(),
        ExecOptions::default(),
    )
    .expect("execute");

    assert_eq!(a.digest_hex(), b.digest_hex());
}

#[test]
fn replay_roundtrip_text_parser() {
    let src = "let x = 5;\nprint x;\n";
    let typed = typecheck(parse_program(src).expect("parse")).expect("typecheck");
    let log = execute_with_defaults(
        typed,
        &ToolRegistry::with_mock_tools(),
        ExecOptions::default(),
    )
    .expect("execute");

    let text = log.to_text();
    let parsed = ReplayLog::from_text(&text).expect("reparse");
    assert_eq!(log.digest_hex(), parsed.digest_hex());
}

#[test]
fn timeout_must_be_non_zero() {
    let src = "tool MockEcho input { value: Int } output { echo: String } cap toolCap;\ncall MockEcho { value: 1 } using toolCap timeout 0;\n";
    let typed = typecheck(parse_program(src).expect("parse")).expect("typecheck");
    let err = execute_with_defaults(
        typed,
        &ToolRegistry::with_mock_tools(),
        ExecOptions::default(),
    )
    .expect_err("expected timeout failure");
    assert!(err.to_string().contains("zero timeout"));
}
