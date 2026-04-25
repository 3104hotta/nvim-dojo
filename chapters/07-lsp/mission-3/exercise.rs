use std::fmt;

pub struct Service {
    name: String,
}

impl Service {
    pub fn new(name: &str) -> Self {
        Service { name: name.to_string() }
    }

    // Rename target: processData → process_data
    pub fn processData(&self, input: &[u8]) -> Vec<u8> {
        let mut out = Vec::with_capacity(input.len());
        for &b in input {
            out.push(b.wrapping_mul(2));
        }
        out
    }

    pub fn run(&self, batches: &[Vec<u8>]) -> Vec<Vec<u8>> {
        batches.iter().map(|b| self.processData(b)).collect()
    }

    // code-action target: try `rust-analyzer`'s suggestion
    pub fn deprecated_method(&self, n: u32) -> u32 {
        n.pow(2)
    }
}

#[derive(Debug)]
pub enum AppError {
    Empty,
    Invalid(String),
}

// Add missing Display impl using LSP code action.
// (Currently AppError lacks fmt::Display.)
