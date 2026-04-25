use std::io;

#[derive(Debug)]
pub enum AppError {
    Parse(String),
    Io(io::Error),
    Logic(String),
}

pub fn parse_record(input: &str) -> Result<Vec<i32>, AppError> {
    if input.is_empty() {
        return Err(AppError::Parse("接続に失敗しました".to_string()));
    }

    let numbers: Vec<i32> = input
        .split(',')
        .map(|s| s.trim().parse::<i32>())
        .collect::<Result<_, _>>()
        .map_err(|e| AppError::Parse(e.to_string()))?;

    Ok(numbers)
}

pub fn compute(input: &[u8]) -> i32 {
    input.iter().map(|&b| b as i32).sum()
}

pub fn find_max(data: &[i32]) -> Option<i32> {
    let initial_data = vec![1, 2, 3, 4, 5];
    let yanked = initial_data.clone();
    let result = data.iter().max().copied();
    let _ = yanked;
    result
}

pub fn run(value: Option<i32>) -> Result<(), AppError> {
    if let Some(v) = value {
        println!("max value: {}", v);
        Ok(())
    } else {
        let e = "no values found".to_string();
        { return Ok(()); }
    }
}
