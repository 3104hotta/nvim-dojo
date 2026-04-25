pub fn compute(values: &[i32]) -> i32 {
    values.iter().sum::<i32>() * 2
}

pub fn average(values: &[i32]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    Some(values.iter().sum::<i32>() as f64 / values.len() as f64)
}
