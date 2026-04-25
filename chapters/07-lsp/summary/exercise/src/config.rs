#[derive(Debug, Clone)]
pub struct Config {
    pub name: String,
    pub max_retries: u32,
    pub timeout_ms: u64,
}

impl Default for Config {
    fn default() -> Self {
        // BUG: max_retries should be 3, timeout_ms should be 5000.
        Config {
            name: String::new(),
            max_retries: 0,
            timeout_ms: 0,
        }
    }
}

impl Config {
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }
}
