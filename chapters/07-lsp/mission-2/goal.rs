use std::collections::HashMap;

pub struct Counter {
    value: u64,
    label: String,
}

impl Counter {
    pub fn new(label: String) -> Self {
        Counter { value: 0, label }
    }

    pub fn inc(&mut self) {
        self.value += 1;
    }

    pub fn label(&self) -> &str {
        &self.label
    }
}

pub fn build_label(prefix: &str) -> String {
    let label: String = prefix.to_string();
    label
}

pub fn use_undefined(undefined_value: i32) -> i32 {
    undefined_value + 1
}

pub fn must_use_result() -> Result<(), std::num::ParseIntError> {
    let _ = "42".parse::<i32>()?;
    println!("done");
    Ok(())
}

pub fn use_after_move() {
    let s = String::from("hello");
    let t = s.clone();
    println!("{} {}", s, t);
}

pub fn run() {
    let mut counter = Counter::new("requests".to_string());
    counter.inc();
    counter.inc();
    println!("{}: {}", counter.label(), counter.value);
}
