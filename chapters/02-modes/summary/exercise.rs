use std::collections::HashMap;

#[derive(Debug)]
pub struct Handler {
    routes: HashMap<String, String>,
    middleware: Vec<String>,
}

impl Handler {
    pub fn new() -> Self {
        Handler {
            routes: HashMap::new(),
            middleware: Vec::new(),
        }
    }

    fn handle_get(&self, path: &str) -> Response {
        match self.routes.get(path) {
            Some(body) => Response::new(200, body),
            None => Response::new(404, "Not Found"),
        }
    }

    fn handle_post(&self, path: &str, body: &str) -> Response {
        if body.is_empty() {
            return Response::new(400, "Bad Request");
        }
        println!("POST {} len={}", path, body.len());
        Response::new(200, "OK")
    }

    fn handle_delete(&self, path: &str) -> Response {
        if self.routes.contains_key(path) {
            Response::new(200, "OK")
        } else {
            Response::new(404, "Not Found")
        }
    }

    pub fn dispatch(&self, method: &str, path: &str, body: Option<&str>) -> Response {
        match method {
            "GET"    => self.handle_get(path),
            "POST"   => self.handle_post(path, body.unwrap_or("")),
            "DELETE" => self.handle_delete(path),
            _        => Response::new(405, "Method Not Allowed"),
        }
    }
}

#[derive(Debug)]
pub struct Response {
    pub status: u16,
    pub body: String,
}

impl Response {
    pub fn new(status: u16, body: &str) -> Self {
        Response { status, body: body.to_string() }
    }
}

pub fn handle_auth(_token: &str) -> Response { todo!() }
pub fn handle_metrics() -> Response { todo!() }
pub fn handle_health() -> Response { todo!() }

// --- tests ---
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_dispatch_get_missing() {
//         let h = Handler::new();
//         let r = h.dispatch("GET", "/foo", None);
//         assert_eq!(r.status, 404);
//     }
//
//     #[test]
//     fn test_dispatch_post_empty_body() {
//         let h = Handler::new();
//         let r = h.dispatch("POST", "/foo", Some(""));
//         assert_eq!(r.status, 400);
//     }
// }
