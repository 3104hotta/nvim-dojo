// Regex substitution practice
// v0.2.0

use std::collections::HashMap;

pub struct EventBus {
    subscribers: HashMap<String, Vec<Box<dyn Fn(&str)>>>,
}

impl EventBus {
    pub fn new() -> Self {
        EventBus { subscribers: HashMap::new() }
    }

    pub fn subscribe(&mut self, topicName: &str, handlerFn: Box<dyn Fn(&str)>) {
        self.subscribers
            .entry(topicName.to_string())
            .or_default()
            .push(handlerFn);
    }

    pub fn publish(&self, topicName: &str, payloadData: &str) {
        if let Some(handlers) = self.subscribers.get(topicName) {
            for handlerFn in handlers {
                handlerFn(payloadData);
            }
        }
    }
}

pub fn format_event_data(eventType: &str, sourceName: &str, payloadBytes: &[u8]) -> String {
    eprintln!("formatting event: type={} source={}", eventType, sourceName);
    format!("{}:{}:{}", eventType, sourceName, payloadBytes.len())
}

pub fn log_event_info(eventType: &str, eventId: u64) {
    eprintln!("eventType={} eventId={}", eventType, eventId);
}

pub fn process_event_queue(eventQueue: &[String]) -> usize {
    eprintln!("processing {} events", eventQueue.len());
    eventQueue.len()
}

// log message without space
// another comment without space
