pub fn validate(input: &str) -> Result<(), String> {
    if input.is_empty() {
        return Err("empty input".to_string());
    }

    if input.contains(' ') {
        return Err("contains space".to_string());
    }

    Ok(())
}

pub fn parse(input: &str) -> Result<Vec<i32>, String> {
    validate(input)?;
    input
        .split(',')
        .map(|s| s.trim().parse::<i32>().map_err(|e| e.to_string()))
        .collect()
}
