// Regex substitution practice
// v0.1.0

use std::collections::HashMap;

pub struct event_bus {
    subscribers: HashMap<String, Vec<Box<dyn Fn(&str)>>>,
}

impl event_bus {
    pub fn new() -> Self {
        event_bus { subscribers: HashMap::new() }
    }

    pub fn subscribe(&mut self, topic_name: &str, handler_fn: Box<dyn Fn(&str)>) {
        self.subscribers
            .entry(topic_name.to_string())
            .or_default()
            .push(handler_fn);
    }

    pub fn publish(&self, topic_name: &str, payload_data: &str) {
        if let Some(handlers) = self.subscribers.get(topic_name) {
            for handler_fn in handlers {
                handler_fn(payload_data);
            }
        }
    }
}

pub fn format_event_data(event_type: &str, source_name: &str, payload_bytes: &[u8]) -> String {
    println!("formatting event: type={} source={}", event_type, source_name);
    format!("{}:{}:{}", event_type, source_name, payload_bytes.len())
}

pub fn log_event_info(event_type: &str, event_id: u64) {
    println!("event_type={} event_id={}", event_type, event_id);
}

pub fn process_event_queue(event_queue: &[String]) -> usize {
    println!("processing {} events", event_queue.len());
    event_queue.len()
}

//log message without space
//another comment without space
