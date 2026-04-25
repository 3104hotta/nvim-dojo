use std::fmt;

#[derive(Debug)]
pub struct Pipeline {
    name: String,
    steps: Vec<Box<dyn Step>>,
    config: PipelineConfig,
}

#[derive(Debug, Clone)]
pub struct PipelineConfig {
    pub max_retries: u32,
    pub timeout_ms: u64,
    pub dry_run: bool,
}

pub trait Step: fmt::Debug {
    fn name(&self) -> &str;
    fn run(&self, input: &str) -> Result<String, StepError>;
}

#[derive(Debug)]
pub enum StepError {
    Skipped,
    Failed(String),
    Timeout,
}

impl fmt::Display for StepError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StepError::Skipped => write!(f, "step skipped"),
            StepError::Failed(msg) => write!(f, "step failed: {}", msg),
            StepError::Timeout => write!(f, "step timed out"),
        }
    }
}

// unwrap() calls to be replaced with ?  (5 places)
pub fn load_config(path: &str) -> PipelineConfig {
    let raw = std::fs::read_to_string(path).unwrap();
    let max_retries: u32 = raw.lines().next().unwrap().parse().unwrap();
    let timeout_ms: u64 = raw.lines().nth(1).unwrap().parse().unwrap();
    let dry_run = raw.lines().nth(2).unwrap() == "true";
    PipelineConfig { max_retries, timeout_ms, dry_run }
}

// String::from("...") to be replaced with "...".to_string()  (3 places)
pub fn default_steps() -> Vec<String> {
    vec![
        String::from("validate"),
        String::from("transform"),
        String::from("publish"),
    ]
}

// match block to be replaced with .unwrap_or_else
pub fn safe_parse(s: &str) -> i32 {
    match s.parse::<i32>() {
        Ok(v) => v,
        Err(_) => panic!("failed to parse: {}", s),
    }
}

// old_name to be renamed to new_name
pub fn old_name(x: i32, y: i32) -> i32 {
    x * y + x + y
}

// pub use re-exports
pub use self::StepError as Error;

// commented code block (6 lines) — remove the `// ` prefix
// #[derive(Debug, Clone)]
// pub struct RetryPolicy {
//     pub max_attempts: u32,
//     pub backoff_ms: u64,
// }
//
// impl Default for RetryPolicy {
//     fn default() -> Self {
//         RetryPolicy { max_attempts: 3, backoff_ms: 500 }
//     }
// }
