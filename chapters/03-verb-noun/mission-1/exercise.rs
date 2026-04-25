use std::io;

#[derive(Debug)]
pub enum AppError {
    Parse(String),
    Io(io::Error),
    Logic(String),
}

pub fn parse_record(input: &str) -> Result<Vec<i32>, AppError> {
    if input.is_empty() {
        return Err(AppError::Parse("error: connection failed".to_string()));
    }

    let numbers: Vec<i32> = input
        .split(',')
        .map(|s| s.trim().parse::<i32>())
        .collect::<Result<_, _>>()
        .map_err(|e| AppError::Parse(e.to_string()))?;

    Ok(numbers)
}

pub fn compute(x: i32, y: i32) -> i32 {
    x + y
}

pub fn find_max(data: &[i32]) -> Option<i32> {
    let initial_data = vec![1, 2, 3, 4, 5];
    let result = initial_data.iter().max().copied();
    result
}

pub fn run(result: Option<i32>) -> Result<(), AppError> {
    if let Some(v) = result {
        println!("max value: {}", v);
        Ok(())
    } else {
        let e = "no values found".to_string();
        { return Err(AppError::Logic(e)); }
    }
}
