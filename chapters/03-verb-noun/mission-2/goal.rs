use std::collections::HashMap;

// Nested structures for text-object practice

pub struct Block {
    id: u32,
    data: Vec<Vec<String>>,
}

impl Block {
    pub fn new(id: u32) -> Self {
        Block { id, data: vec![] }
    }

    fn inner(&self) -> String {
        format!("block-{}", self.id)
    }

    pub fn describe(&self) -> String {
        let label = self.inner();
        format!("[{}] cells={}", label, self.data.len())
    }
}

pub fn transform(result: Result<i32, String>) -> i32 {
    match result {
        Ok(v) if v > 0 => v * 2,
        Ok(_) => 0,
        Err(e) => {
            eprintln!("error: {}", e);
            -1
        }
    }
}

pub fn flatten(matrix: Vec<Vec<Vec<i32>>>) -> Vec<i32> {
    matrix.into_iter()
        .flat_map(|plane| plane.into_iter())
        .flat_map(|row| row.into_iter())
        .collect()
}

pub fn log_value(v: i64) {
    println!("value: {}", v);
}

pub fn build_index(items: &[&str]) -> HashMap<&str, usize> {
    let mut index = HashMap::new();

    for (i, &item) in items.iter().enumerate() {
        index.insert(item, i);
    }

    index
}

pub fn summarize(counts: &HashMap<String, u32>) -> String {
    let total: u32 = counts.values().sum();
    let entries: Vec<String> = counts
        .iter()
        .map(|(k, v)| format!("{}:{}", k, v))
        .collect();
    format!("total={} [{}]", total, entries.join(", "))
}
