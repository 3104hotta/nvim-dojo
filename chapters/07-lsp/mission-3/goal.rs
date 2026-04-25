use std::fmt;

pub struct Service {
    name: String,
}

impl Service {
    pub fn new(name: &str) -> Self {
        Service { name: name.to_string() }
    }

    pub fn process_data(&self, input: &[u8]) -> Vec<u8> {
        let mut out = Vec::with_capacity(input.len());
        for &b in input {
            out.push(b.wrapping_mul(2));
        }
        out
    }

    pub fn run(&self, batches: &[Vec<u8>]) -> Vec<Vec<u8>> {
        batches.iter().map(|b| self.process_data(b)).collect()
    }

    pub fn squared(&self, n: u32) -> u32 {
        n.pow(2)
    }
}

#[derive(Debug)]
pub enum AppError {
    Empty,
    Invalid(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Empty       => write!(f, "empty"),
            AppError::Invalid(m)  => write!(f, "invalid: {}", m),
        }
    }
}
