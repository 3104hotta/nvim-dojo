use crate::handler::{Handler, Response};
use std::collections::HashMap;

pub struct Router<H: Handler> {
    handler: H,
    routes: HashMap<String, ()>,
}

impl<H: Handler> Router<H> {
    pub fn new(handler: H) -> Self {
        Router { handler, routes: HashMap::new() }
    }

    // Implement: pub fn add_route(&mut self, method: &str, path: &str)
    // Implement: pub fn dispatch(&self, method: &str, path: &str, body: Option<&str>) -> Response
}
