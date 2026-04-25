// Search/replace mix:
//   1. timeout_ms -> timeout_millis (word boundary)
//   2. trailing whitespace
//   3. "debug" -> "trace" (4 places, n.)

#[derive(Debug, Clone)]
pub struct Config {
    pub timeout_ms: u64,
    pub idle_timeout_ms: u64,
    pub max_timeout_ms: u64,
    pub log_level: &'static str,
}

impl Config {
    pub fn timeout_ms(&self) -> u64 {
        self.timeout_ms
    }

    pub fn with_timeout_ms(mut self, timeout_ms: u64) -> Self {
        self.timeout_ms = timeout_ms;
        self
    }
}

pub fn default_log_levels() -> Vec<&'static str> {
    vec!["debug", "debug", "debug", "debug"]
}

pub fn describe(cfg: &Config) -> String {
    format!(
        "timeout_ms={} idle={} max={} level={}",
        cfg.timeout_ms, cfg.idle_timeout_ms, cfg.max_timeout_ms, cfg.log_level
    )
}
