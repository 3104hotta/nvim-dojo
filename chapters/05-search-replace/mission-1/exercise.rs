use std::collections::HashMap;

// Search navigation practice
// "handler" appears 6 times — find them all with * then replace with "router" using n.

pub struct handler {
    routes: HashMap<String, Box<dyn Fn(&str) -> String>>,
}

impl handler {
    pub fn new() -> Self {
        handler { routes: HashMap::new() }
    }

    pub fn register(&mut self, path: String, f: Box<dyn Fn(&str) -> String>) {
        self.routes.insert(path, f);
    }

    pub fn dispatch(&self, path: &str) -> Option<String> {
        self.routes.get(path).map(|f| f(path))
    }
}

pub fn build_handler() -> handler {
    handler::new()
}

pub fn default_handler() -> Box<dyn Fn(&str) -> String> {
    Box::new(|_| "not found".to_string())
}

// backslash in string — search with /\\
pub const PATH_SEP: &str = "C:\\Users\\app\\data";
pub const ESCAPE_SEQ: &str = "newline=\\n, tab=\\t";

// mixed case — search with /error\c
pub fn handle_Error(msg: &str) {
    eprintln!("ERROR: {}", msg);
}

pub fn log_error(msg: &str) {
    eprintln!("error: {}", msg);
}

pub fn raise_ERROR(code: u32) -> String {
    format!("Error code: {}", code)
}
