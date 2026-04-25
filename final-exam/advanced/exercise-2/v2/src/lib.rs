// v2 — new style: derive macros, ? + thiserror-style errors, no panics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    pub id: u64,
    pub name: String,
    pub tags: Vec<String>,
}

#[derive(Debug)]
pub enum AppError {
    Empty,
    Invalid(String),
    Parse(std::num::ParseIntError),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Empty      => write!(f, "empty"),
            AppError::Invalid(m) => write!(f, "invalid: {}", m),
            AppError::Parse(e)   => write!(f, "parse: {}", e),
        }
    }
}

impl std::error::Error for AppError {}

impl From<std::num::ParseIntError> for AppError {
    fn from(e: std::num::ParseIntError) -> Self { AppError::Parse(e) }
}

pub fn parse(input: &str) -> Result<Record, AppError> {
    if input.is_empty() {
        return Err(AppError::Empty);
    }
    let parts: Vec<&str> = input.split(':').collect();
    if parts.len() < 2 {
        return Err(AppError::Invalid(input.to_string()));
    }
    Ok(Record {
        id: parts[0].parse()?,
        name: parts[1].to_string(),
        tags: vec![],
    })
}

pub fn parse_with_tags(input: &str) -> Result<Record, AppError> {
    let mut record = parse(input)?;
    if let Some(rest) = input.split(':').nth(2) {
        record.tags = rest.split(',').map(String::from).collect();
    }
    Ok(record)
}
