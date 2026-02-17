use std::collections::HashMap;

pub type ToolFn = Box<dyn Fn(String) -> Result<String, String> + Send + Sync>;

#[derive(Default)]
pub struct ToolRegistry {
    tools: HashMap<String, ToolFn>,
}

impl ToolRegistry {
    pub fn with_mock_tools() -> Self {
        let mut reg = Self::default();
        reg.register(
            "MockEcho",
            Box::new(|input| Ok(format!("{{\"echo\":{input}}}"))),
        );
        reg
    }
    pub fn register(&mut self, name: &str, f: ToolFn) {
        self.tools.insert(name.to_string(), f);
    }
    pub fn call(&self, name: &str, input: String) -> Result<String, String> {
        self.tools
            .get(name)
            .ok_or_else(|| format!("unknown tool: {name}"))?(input)
    }
}
