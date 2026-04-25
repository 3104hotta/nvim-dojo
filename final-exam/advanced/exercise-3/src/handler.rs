pub struct Response {
    pub status: u16,
    pub body: String,
}

impl Response {
    pub fn ok(body: &str) -> Self {
        Response { status: 200, body: body.to_string() }
    }
}

pub trait Handler {
    fn handle_get(&self, path: &str) -> Response;
    fn handle_post(&self, path: &str, body: &str) -> Response;
}

pub struct EchoHandler;

// Implement: impl Handler for EchoHandler { ... }
