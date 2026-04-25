use std::collections::HashMap;
use std::io::Read;  // unused import — error 5

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
    // error 1: type mismatch — expects String, given &str
    let label: String = prefix;
    label
}

pub fn use_undefined() -> i32 {
    // error 2: undefined variable
    undefined_value + 1
}

pub fn must_use_result() {
    // error 3: must_use Result not handled
    "42".parse::<i32>();
    println!("done");
}

pub fn use_after_move() {
    let s = String::from("hello");
    let _t = s;
    // error 4: borrow after move
    println!("{}", s);
}

pub fn run() {
    let mut counter = Counter::new("requests".to_string());
    counter.inc();
    counter.inc();
    println!("{}: {}", counter.label(), counter.value);
}
