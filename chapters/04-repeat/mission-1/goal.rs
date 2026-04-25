use std::fs;
use std::io;

pub fn read_file(path: &str) -> Result<String, io::Error> {
    let content = fs::read_to_string(path)?;
    Ok(content)
}

pub fn parse_int(s: &str) -> Result<i32, String> {
    let n = s.trim().parse::<i32>().map_err(|e| e.to_string())?;
    Ok(n)
}

pub fn first_line(path: &str) -> Result<String, io::Error> {
    let content = fs::read_to_string(path)?;
    let line = content.lines().next().unwrap_or("").to_string();
    Ok(line)
}

pub fn load_lines(path: &str) -> Result<Vec<String>, io::Error> {
    Ok(fs::read_to_string(path)?
        .lines()
        .map(String::from)
        .collect())
}

pub fn a() -> i32 { 1 }
pub fn b() -> i32 { 2 }
pub fn c() -> i32 { 3 }
pub fn d() -> i32 { 4 }
pub fn e() -> i32 { 5 }
