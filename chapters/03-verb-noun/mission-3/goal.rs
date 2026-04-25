use std::fmt;

// Register practice: yank/paste with named registers

pub fn alpha(x: i32) -> i32 {
    let doubled = x * 2;
    let shifted = doubled + 1;
    shifted
}

pub fn alpha_copy(x: i32) -> i32 {
    let doubled = x * 2;
    let shifted = doubled + 1;
    shifted
}

pub fn beta(x: i32) -> Result<String, String> {
    if x < 0 {
        return Err(format!("negative input: {}", x));
    }
    Ok(format!("value={}", x))
}

pub fn gamma(x: i32) -> i32 {
    x * x
}

pub fn delta(x: i32) -> Result<String, String> {
    println!("delta: {}", x);
    Ok(format!("value={}", x))
}

pub fn public_api(input: &str) -> Result<Vec<i32>, String> {
    let msg = "v";
    let items: Vec<i32> = input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    println!("{}: parsed {} items", msg, items.len());
    Ok(items)
}

#[derive(Debug)]
pub enum Status {
    Active,
    Inactive,
    Pending(String),
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Active => write!(f, "active"),
            Status::Inactive => write!(f, "inactive"),
            Status::Pending(reason) => write!(f, "pending: {}", reason),
        }
    }
}
