use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone)]
pub struct Capability {
    id: u64,
    kind: CapabilityKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CapabilityKind {
    Tool,
    Net,
    Fs,
    Memory,
}

impl Capability {
    pub fn can_use_tool(&self) -> bool {
        self.kind == CapabilityKind::Tool
    }

    pub fn narrow_to_tool(&self) -> Option<Self> {
        if self.can_use_tool() {
            Some(Self {
                id: self.id,
                kind: CapabilityKind::Tool,
            })
        } else {
            None
        }
    }
}

fn mint(kind: CapabilityKind) -> Capability {
    Capability {
        id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
        kind,
    }
}

pub fn default_capabilities() -> HashMap<String, Capability> {
    let mut caps = HashMap::new();
    caps.insert("toolCap".to_string(), mint(CapabilityKind::Tool));
    caps
}
