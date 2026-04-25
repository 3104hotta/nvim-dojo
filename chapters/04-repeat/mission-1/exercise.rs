use std::fs;
use std::io;

// Dot-repeat practice
// Task 1: replace unwrap() with ? (4 places) using n.
// Task 2: add (crate) to pub fn (3 places) using n.
// Task 3: delete trailing ; on marked lines (5 lines) using j.

pub fn read_file(path: &str) -> Result<String, io::Error> {
    let content = fs::read_to_string(path).unwrap();
    Ok(content)
}

pub fn parse_int(s: &str) -> Result<i32, String> {
    let n = s.trim().parse::<i32>().unwrap();
    Ok(n)
}

pub fn first_line(path: &str) -> Result<String, io::Error> {
    let content = fs::read_to_string(path).unwrap();
    let line = content.lines().next().unwrap().to_string();
    Ok(line)
}

pub fn load_lines(path: &str) -> Result<Vec<String>, io::Error> {
    Ok(fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect())
}

// [SEMI] remove trailing ; from these 5 lines
pub fn a() -> i32 { 1 };
pub fn b() -> i32 { 2 };
pub fn c() -> i32 { 3 };
pub fn d() -> i32 { 4 };
pub fn e() -> i32 { 5 };
