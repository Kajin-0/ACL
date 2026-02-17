use crate::typecheck::TypedProgram;

pub fn render_manifest(typed: &TypedProgram) -> String {
    let mut out = String::new();
    out.push_str("capability_manifest_v1\n");
    for cap in &typed.manifest.required_caps {
        out.push_str(&format!("requires={cap}\n"));
    }
    out
}
