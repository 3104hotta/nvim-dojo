// Macro practice
// Section A: wrap each tuple entry with Config::new(...)  (10 lines)
// Section B: append .unwrap_or(0) to each compute() call  (8 lines)

// ---- Section A ----
pub fn default_settings() -> Vec<(&'static str, &'static str)> {
    vec![
        ("host", "localhost"),
        ("port", "8080"),
        ("timeout", "30"),
        ("max_conn", "100"),
        ("log_level", "info"),
        ("data_dir", "/var/data"),
        ("cache_ttl", "300"),
        ("retry", "3"),
        ("backoff", "500"),
        ("dry_run", "false"),
    ]
}

// ---- Section B ----
pub fn compute(n: i32) -> Option<i32> {
    if n > 0 { Some(n * 2) } else { None }
}

pub fn run_all() -> Vec<i32> {
    vec![
        compute(1),
        compute(2),
        compute(3),
        compute(4),
        compute(5),
        compute(6),
        compute(7),
        compute(8),
    ]
}
