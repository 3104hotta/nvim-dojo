use old_crate::Result;
use std::collections::HashMap;

// v1.x

pub struct Router {
    routes: HashMap<String, String>,
}

impl Router {
    pub fn new() -> Self {
        Router { routes: HashMap::new() }
    }

    pub fn add(&mut self, path: &str, target: &str) {
        self.routes.insert(path.to_string(), target.to_string());
    }

    pub fn resolve(&self, path: &str) -> Result<&str> {
        self.routes
            .get(path)
            .map(|s| s.as_str())
            .ok_or_else(|| old_crate::Error::not_found())
    }
}
