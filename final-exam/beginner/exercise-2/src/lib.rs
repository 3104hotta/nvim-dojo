use std::io;

pub struct Builder {
    parts: Vec<String>,
}

impl Builder {
    pub fn new() -> Self {
        Builder { parts: vec![] }
    }

    pub fn push(&mut self, name: &str) {
        self.parts.push(String::from(name));
    }

    pub fn header(name: &str) -> String {
        String::from("Header: ") + &String::from(name)
    }

    pub fn build(self) -> String {
        self.parts.join("/")
    }
}

pub fn read_first(path: &str) -> Result<String, io::Error> {
    let s = std::fs::read_to_string(path).unwrap();
    let line = s.lines().next().unwrap().to_string();
    Ok(line)
}

pub fn count_words(path: &str) -> usize {
    let content = std::fs::read_to_string(path).unwrap();
    content.split_whitespace().count()
}

pub fn extract_first_int(path: &str) -> i32 {
    let content = std::fs::read_to_string(path).unwrap();
    content.parse::<i32>().unwrap()
}

pub fn defaults() -> Vec<i32> {
    let primary = vec![1, 2, 3, 4, 5];
    let _secondary: Vec<i32> = vec![];
    primary
}
