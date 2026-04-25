// v1 — old style: manual error handling, manual derives, panicking helpers.
use std::fmt;

pub struct Record {
    pub id: u64,
    pub name: String,
    pub tags: Vec<String>,
}

impl Clone for Record {
    fn clone(&self) -> Self {
        Record {
            id: self.id,
            name: self.name.clone(),
            tags: self.tags.clone(),
        }
    }
}

impl PartialEq for Record {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name && self.tags == other.tags
    }
}

impl fmt::Debug for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Record {{ id: {}, name: {}, tags: {:?} }}",
            self.id, self.name, self.tags)
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

pub fn parse(input: &str) -> Record {
    if input.is_empty() {
        panic!("empty input");
    }
    let parts: Vec<&str> = input.split(':').collect();
    if parts.len() < 2 {
        panic!("malformed input: {}", input);
    }
    Record {
        id: parts[0].parse().unwrap(),
        name: parts[1].to_string(),
        tags: vec![],
    }
}
