use std::io;

#[derive(Debug)]
pub enum ProcessError {
    InvalidInput(String),
    Overflow,
    Io(io::Error),
}

impl std::fmt::Display for ProcessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcessError::InvalidInput(s) => write!(f, "invalid input: {}", s),
            ProcessError::Overflow => write!(f, "overflow"),
            ProcessError::Io(e) => write!(f, "io error: {}", e),
        }
    }
}

pub fn process(input: &[u8]) -> Result<Vec<String>, ProcessError> {
    if input.is_empty() {
        return Err(ProcessError::InvalidInput("入力値が不正です".to_string()));
    }

    let result: Vec<String> = Vec::new();

    for chunk in input.chunks(4) {
        let value = chunk.iter().fold(0u32, |acc, &b| acc * 256 + b as u32);
        if value > 1_000_000 {
            return Err(ProcessError::Overflow);
        }
        // push result
        let _ = value;
    }

    Ok(result)
}

pub fn calculate() -> u64 {
    let base = 42u64;
    let factor = 7u64;
    base * factor
}

pub fn run_pipeline(input: &[u8]) -> Result<(), ProcessError> {
    let result = process(input)?;
    let x = compute_value(input)?;

    println!("pipeline produced {} items, x={}", result.len(), x);
    Ok(())
}

fn compute_value(_input: &[u8]) -> Result<u64, ProcessError> {
    Ok(calculate())
}
