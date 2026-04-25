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

pub fn load_config(path: &str) -> Result<PipelineConfig, Box<dyn std::error::Error>> {
    let raw = std::fs::read_to_string(path)?;
    let max_retries: u32 = raw.lines().next().ok_or("missing line 1")?.parse()?;
    let timeout_ms: u64 = raw.lines().nth(1).ok_or("missing line 2")?.parse()?;
    let dry_run = raw.lines().nth(2).ok_or("missing line 3")? == "true";
    Ok(PipelineConfig { max_retries, timeout_ms, dry_run })
}

pub fn default_steps() -> Vec<String> {
    vec![
        "validate".to_string(),
        "transform".to_string(),
        "publish".to_string(),
    ]
}

pub fn safe_parse(s: &str) -> i32 {
    s.parse::<i32>().unwrap_or_else(|_| panic!("failed to parse: {}", s))
}

pub fn new_name(x: i32, y: i32) -> i32 {
    x * y + x + y
}

pub use self::StepError as Error;

#[derive(Debug, Clone)]
pub struct RetryPolicy {
    pub max_attempts: u32,
    pub backoff_ms: u64,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        RetryPolicy { max_attempts: 3, backoff_ms: 500 }
    }
}
