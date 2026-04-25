use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct Handler {
    routes: HashMap<String, String>,
}

impl Handler {
    pub fn new() -> Self {
        Handler { routes: HashMap::new() }
    }

    pub fn add(&mut self, path: &str, target: &str) {
        self.routes.insert(path.to_string(), target.to_string());
    }
}

#[derive(Debug)]
pub struct Request {
    pub path: String,
    pub method: String,
    pub body: Vec<u8>,
}

#[derive(Debug)]
pub struct Response {
    pub status: u16,
    pub body: Vec<u8>,
}

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    BadRequest(String),
    Internal(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound(p)  => write!(f, "not found: {}", p),
            AppError::BadRequest(m) => write!(f, "bad request: {}", m),
            AppError::Internal(m)   => write!(f, "internal: {}", m),
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::Internal(e.to_string())
    }
}

pub fn process_request(handler: &Handler, req: Request) -> Result<Response, AppError> {
    let target = handler
        .routes
        .get(&req.path)
        .ok_or_else(|| AppError::NotFound(req.path.clone()))?;

    Ok(Response {
        status: 200,
        body: format!("dispatched to {}", target).into_bytes(),
    })
}

pub fn run_server(handler: &Handler, requests: Vec<Request>) -> Vec<Result<Response, AppError>> {
    requests.into_iter().map(|req| process_request(handler, req)).collect()
}

fn main() {
    let mut handler = Handler::new();
    handler.add("/", "index");
    handler.add("/about", "about-page");

    let req = Request {
        path: "/".to_string(),
        method: "GET".to_string(),
        body: vec![],
    };

    let _ = process_request(&handler, req);
}
