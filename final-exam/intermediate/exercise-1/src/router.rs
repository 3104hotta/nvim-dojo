//! router module
use log::debug;
use std::collections::HashMap;

pub struct Router {
    routes: HashMap<String, String>,
}

impl Router {
    pub fn new() -> Self {
        log::debug!("router init");
        Router { routes: HashMap::new() }
    }

    pub fn add(&mut self, path: &str, target: &str) {
        log::info!("add route {} -> {}", path, target);
        self.routes.insert(path.to_string(), target.to_string());
    }
}
