// Macro practice — goal state

pub struct Config(pub &'static str, pub &'static str);

impl Config {
    pub fn new(k: &'static str, v: &'static str) -> Self {
        Config(k, v)
    }
}

pub fn default_settings() -> Vec<Config> {
    vec![
        Config::new("host", "localhost"),
        Config::new("port", "8080"),
        Config::new("timeout", "30"),
        Config::new("max_conn", "100"),
        Config::new("log_level", "info"),
        Config::new("data_dir", "/var/data"),
        Config::new("cache_ttl", "300"),
        Config::new("retry", "3"),
        Config::new("backoff", "500"),
        Config::new("dry_run", "false"),
    ]
}

pub fn compute(n: i32) -> Option<i32> {
    if n > 0 { Some(n * 2) } else { None }
}

pub fn run_all() -> Vec<i32> {
    vec![
        compute(1).unwrap_or(0),
        compute(2).unwrap_or(0),
        compute(3).unwrap_or(0),
        compute(4).unwrap_or(0),
        compute(5).unwrap_or(0),
        compute(6).unwrap_or(0),
        compute(7).unwrap_or(0),
        compute(8).unwrap_or(0),
    ]
}
