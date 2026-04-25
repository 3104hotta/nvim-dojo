use std::collections::HashMap;
use std::fmt;
use std::io;
use std::time::Duration;
use url::Url;

// HTTP client library
// NOTE: refactor candidate

pub struct HttpClient {
    base_url: Url,
    timeout: Duration,
    headers: HashMap<String, String>,
    max_retries: u32,
}

impl HttpClient {
    pub fn new(base_url: Url) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("User-Agent".to_string(), "nvim-dojo-client/1.0".to_string());

        HttpClient {
            base_url,
            timeout: Duration::from_secs(30),
            headers,
            max_retries: 3,
        }
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    pub fn get(&self, path: &str) -> Result<Response, HttpError> {
        let url = format!("{}{}", self.base_url, path);
        println!("GET {}", url);
        todo!()
    }

    pub fn post(&self, path: &str, body: &str) -> Result<Response, HttpError> {
        let url = format!("{}{}", self.base_url, path);
        println!("POST {} body_len={}", url, body.len());
        todo!()
    }

    pub fn put(&self, path: &str, body: &str) -> Result<Response, HttpError> {
        let url = format!("{}{}", self.base_url, path);
        println!("PUT {} body_len={}", url, body.len());
        todo!()
    }

    pub fn delete(&self, path: &str) -> Result<Response, HttpError> {
        let url = format!("{}{}", self.base_url, path);
        println!("DELETE {}", url);
        todo!()
    }

    fn retry<F>(&self, mut f: F) -> Result<Response, HttpError>
    where
        F: FnMut() -> Result<Response, HttpError>,
    {
        let mut last_err = None;
        for attempt in 0..self.max_retries {
            match f() {
                Ok(resp) => return Ok(resp),
                Err(e) => {
                    eprintln!("Attempt {} failed: {}", attempt + 1, e);
                    last_err = Some(e);
                }
            }
        }
        Err(last_err.unwrap_or(HttpError::Unknown))
    }
}

#[derive(Debug)]
pub struct Response {
    pub status: u16,
    pub body: String,
    pub headers: HashMap<String, String>,
}

impl Response {
    pub fn new(status: u16, body: &str) -> Self {
        Response {
            status,
            body: body.to_string(),
            headers: HashMap::new(),
        }
    }

    pub fn is_success(&self) -> bool {
        self.status >= 200 && self.status < 300
    }

    pub fn is_client_error(&self) -> bool {
        self.status >= 400 && self.status < 500
    }

    pub fn is_server_error(&self) -> bool {
        self.status >= 500
    }
}

#[derive(Debug)]
pub enum HttpError {
    Timeout,
    ConnectionRefused,
    TooManyRetries { attempts: u32 },
    InvalidUrl(String),
    ServerError { status: u16, message: String },
    Unknown,
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpError::Timeout => write!(f, "request timed out"),
            HttpError::ConnectionRefused => write!(f, "connection refused"),
            HttpError::TooManyRetries { attempts } => {
                write!(f, "failed after {} attempts", attempts)
            }
            HttpError::InvalidUrl(url) => write!(f, "invalid URL: {}", url),
            HttpError::ServerError { status, message } => {
                write!(f, "server error {}: {}", status, message)
            }
            HttpError::Unknown => write!(f, "unknown error"),
        }
    }
}

impl From<io::Error> for HttpError {
    fn from(e: io::Error) -> Self {
        match e.kind() {
            io::ErrorKind::TimedOut => HttpError::Timeout,
            io::ErrorKind::ConnectionRefused => HttpError::ConnectionRefused,
            _ => HttpError::Unknown,
        }
    }
}

pub fn build_query_string(params: &[(&str, &str)]) -> String {
    params
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<_>>()
        .join("&")
}

// reviewed by: <your-name>

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_query_string() {
        let params = vec![("foo", "bar"), ("baz", "qux")];
        let qs = build_query_string(&params);
        assert_eq!(qs, "foo=bar&baz=qux");
    }

    #[test]
    fn test_response_status() {
        let r = Response::new(200, "ok");
        assert!(r.is_success());
        let r = Response::new(404, "not found");
        assert!(r.is_client_error());
        let r = Response::new(500, "error");
        assert!(r.is_server_error());
    }
}
