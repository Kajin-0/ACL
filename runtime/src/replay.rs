#[derive(Debug, Clone)]
pub enum Event {
    Print {
        value: String,
    },
    ToolCall {
        tool: String,
        input: String,
        output: String,
        source: String,
        timestamp_ms: u64,
        output_hash: String,
        policy_tags: Vec<String>,
    },
    Random {
        value: u64,
    },
    Time {
        millis: u64,
    },
}

#[derive(Debug, Default, Clone)]
pub struct ReplayLog {
    pub events: Vec<Event>,
}

impl ReplayLog {
    pub fn push(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn digest_hex(&self) -> String {
        let serialized = self.to_text();
        fnv1a64_hex(serialized.as_bytes())
    }

    pub fn to_text(&self) -> String {
        let mut out = String::new();
        for e in &self.events {
            out.push_str(&event_to_line(e));
            out.push('\n');
        }
        out
    }

    pub fn from_text(s: &str) -> Result<Self, String> {
        let mut events = Vec::new();
        for (idx, line) in s.lines().enumerate() {
            if line.trim().is_empty() {
                continue;
            }
            events.push(parse_line(line).map_err(|e| format!("line {}: {e}", idx + 1))?);
        }
        Ok(Self { events })
    }
}

fn event_to_line(e: &Event) -> String {
    match e {
        Event::Print { value } => format!("PRINT|{}", escape(value)),
        Event::ToolCall {
            tool,
            input,
            output,
            source,
            timestamp_ms,
            output_hash,
            policy_tags,
        } => format!(
            "TOOL|{}|{}|{}|{}|{}|{}|{}",
            escape(tool),
            escape(input),
            escape(output),
            escape(source),
            timestamp_ms,
            escape(output_hash),
            escape(&policy_tags.join(","))
        ),
        Event::Random { value } => format!("RANDOM|{value}"),
        Event::Time { millis } => format!("TIME|{millis}"),
    }
}

fn parse_line(line: &str) -> Result<Event, String> {
    let parts: Vec<&str> = line.split('|').collect();
    match parts.first().copied().unwrap_or_default() {
        "PRINT" if parts.len() == 2 => Ok(Event::Print {
            value: unescape(parts[1]),
        }),
        "TOOL" if parts.len() == 8 => Ok(Event::ToolCall {
            tool: unescape(parts[1]),
            input: unescape(parts[2]),
            output: unescape(parts[3]),
            source: unescape(parts[4]),
            timestamp_ms: parts[5]
                .parse::<u64>()
                .map_err(|_| "invalid timestamp".to_string())?,
            output_hash: unescape(parts[6]),
            policy_tags: unescape(parts[7])
                .split(',')
                .filter(|s| !s.is_empty())
                .map(ToString::to_string)
                .collect(),
        }),
        "RANDOM" if parts.len() == 2 => Ok(Event::Random {
            value: parts[1]
                .parse::<u64>()
                .map_err(|_| "invalid random value".to_string())?,
        }),
        "TIME" if parts.len() == 2 => Ok(Event::Time {
            millis: parts[1]
                .parse::<u64>()
                .map_err(|_| "invalid millis".to_string())?,
        }),
        _ => Err("invalid replay event".to_string()),
    }
}

fn escape(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('|', "\\|")
        .replace('\n', "\\n")
}

fn unescape(s: &str) -> String {
    let mut out = String::new();
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') => out.push('\n'),
                Some(other) => out.push(other),
                None => out.push('\\'),
            }
        } else {
            out.push(c);
        }
    }
    out
}

fn fnv1a64_hex(bytes: &[u8]) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for b in bytes {
        hash ^= *b as u64;
        hash = hash.wrapping_mul(0x00000100000001B3);
    }
    format!("{hash:016x}")
}
