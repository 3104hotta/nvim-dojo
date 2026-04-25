use std::fmt;

/// A task in the pipeline.
/// Holds the input data and processing state.
/// Errors are propagated via Result.
/// # Example
pub struct Task {
    pub id: u64,
    pub payload: Vec<u8>,
}

/// Processes a single task.
/// Returns the output bytes on success.
/// May fail if the payload is malformed.
/// # Example
impl Task {
    pub fn run(&self) -> Result<Vec<u8>, AppError> {
        let out = self.payload
            .iter()
            .map(|&b| b.wrapping_add(1))
            .collect();
        Ok(out)
    }
}

/// Validates task payload length.
/// Returns true when valid.
/// Empty payloads are rejected.
/// # Example
pub fn validate(task: &Task) -> bool {
    !task.payload.is_empty() && task.payload.len() < 65536
}

/// Serialises a task to JSON-like string.
/// Used for debugging and logging.
/// Not for production serialisation.
/// # Example
pub fn serialize(task: &Task) -> String {
    format!(r#"{{"id":{},"len":{}}}"#, task.id, task.payload.len())
}

/// Loads tasks from raw bytes.
/// Each task is 8 bytes header + N bytes payload.
/// Returns empty vec on malformed input.
/// # Example
pub fn load(data: &[u8]) -> Vec<Task> {
    let mut tasks = Vec::new();
    let mut i = 0;
    while i + 8 <= data.len() {
        let id = u64::from_le_bytes(data[i..i+8].try_into().unwrap());
        tasks.push(Task { id, payload: data[i+8..].to_vec() });
        i += 8;
    }
    tasks
}

#[derive(Debug)]
pub enum AppError {
    Empty,
    TooLarge,
    Io(std::io::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Empty    => write!(f, "empty payload"),
            AppError::TooLarge => write!(f, "payload too large"),
            AppError::Io(e)    => write!(f, "io: {}", e),
        }
    }
}

pub fn wrap_1(e: std::io::Error) -> Box<AppError> { AppError::from(e).into() }
pub fn wrap_2(e: std::io::Error) -> Box<AppError> { AppError::from(e).into() }
pub fn wrap_3(e: std::io::Error) -> Box<AppError> { AppError::from(e).into() }
pub fn wrap_4(e: std::io::Error) -> Box<AppError> { AppError::from(e).into() }
pub fn wrap_5(e: std::io::Error) -> Box<AppError> { AppError::from(e).into() }

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self { AppError::Io(e) }
}

pub fn step_a(data: &[u8]) -> Result<Vec<u8>, AppError> {
    std::fs::read("/tmp/a").map_err(|e| { tracing::error!(err = %e); e }).map_err(AppError::Io)?;
    Ok(data.to_vec())
}
pub fn step_b(data: &[u8]) -> Result<Vec<u8>, AppError> {
    std::fs::read("/tmp/b").map_err(|e| { tracing::error!(err = %e); e }).map_err(AppError::Io)?;
    Ok(data.to_vec())
}
pub fn step_c(data: &[u8]) -> Result<Vec<u8>, AppError> {
    std::fs::read("/tmp/c").map_err(|e| { tracing::error!(err = %e); e }).map_err(AppError::Io)?;
    Ok(data.to_vec())
}
