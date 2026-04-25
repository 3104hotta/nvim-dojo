// Center pane — wide (target: 50 columns)
// This is the main editing area where most of the work happens.

pub fn process(input: &str) -> String {
    input.chars().rev().collect()
}

pub fn validate(input: &str) -> bool {
    !input.is_empty() && input.len() < 1024
}

pub fn transform(input: &str) -> String {
    input.to_uppercase()
}
