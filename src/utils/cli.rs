use std::collections::HashMap;

#[derive(Debug)]
pub struct ArgParser {
    pub args: Vec<String>,
}

impl ArgParser {
    pub fn new() -> Self {
        ArgParser {
            args: std::env::args().skip(1).collect(),
        }
    }

    pub fn parse(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        let mut iter = self.args.iter();
        while let Some(arg) = iter.next() {
            if arg.starts_with("--") {
                if let Some(value) = iter.next() {
                    map.insert(arg.trim_start_matches("--").to_string(), value.to_string());
                }
            }
        }
        map
    }

    pub fn require(&self, key: &str) -> Result<String, String> {
        let map = self.parse();
        map.get(key)
            .cloned()
            .ok_or_else(|| format!("missing required argument --{key}"))
    }

    pub fn optional(&self, key: &str, default: &str) -> String {
        let map = self.parse();
        map.get(key)
            .cloned()
            .unwrap_or_else(|| default.to_string())
    }
}
