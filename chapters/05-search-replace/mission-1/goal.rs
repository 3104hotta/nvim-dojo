use std::collections::HashMap;

pub struct router {
    routes: HashMap<String, Box<dyn Fn(&str) -> String>>,
}

impl router {
    pub fn new() -> Self {
        router { routes: HashMap::new() }
    }

    pub fn register(&mut self, path: String, f: Box<dyn Fn(&str) -> String>) {
        self.routes.insert(path, f);
    }

    pub fn dispatch(&self, path: &str) -> Option<String> {
        self.routes.get(path).map(|f| f(path))
    }
}

pub fn build_router() -> router {
    router::new()
}

pub fn default_router() -> Box<dyn Fn(&str) -> String> {
    Box::new(|_| "not found".to_string())
}

pub const PATH_SEP: &str = "C:\\Users\\app\\data";
pub const ESCAPE_SEQ: &str = "newline=\\n, tab=\\t";

pub fn handle_Error(msg: &str) {
    eprintln!("ERROR: {}", msg);
}

pub fn log_error(msg: &str) {
    eprintln!("error: {}", msg);
}

pub fn raise_ERROR(code: u32) -> String {
    format!("Error code: {}", code)
}
