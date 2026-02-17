use axiom_compiler::{
    manifest::render_manifest, parser::parse_program, typecheck::typecheck, Effect,
};

#[test]
fn parses_and_typechecks_pure_program() {
    let src = "let x = 1 + 2;\nprint x;\n";
    let p = parse_program(src).expect("parse");
    let typed = typecheck(p).expect("typecheck");
    assert_eq!(typed.effect, Effect::Pure);
}

#[test]
fn detects_tool_effect_and_manifest() {
    let src = "tool MockEcho input { message: String } output { echo: String } cap toolCap;\ncall MockEcho { message: \"hi\" } using toolCap timeout 1000;";
    let p = parse_program(src).expect("parse");
    let typed = typecheck(p).expect("typecheck");
    assert_eq!(typed.effect, Effect::Tool);
    let manifest = render_manifest(&typed);
    assert!(manifest.contains("requires=toolCap"));
}

#[test]
fn rejects_mismatched_tool_capability() {
    let src = "tool MockEcho input { message: String } output { echo: String } cap toolCap;\ncall MockEcho { message: \"hi\" } using badCap timeout 1000;";
    let p = parse_program(src).expect("parse");
    let err = typecheck(p).expect_err("expected cap mismatch");
    assert!(err.to_string().contains("requires cap"));
}
