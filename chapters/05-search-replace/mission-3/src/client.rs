// TODO: implement retry logic
// TODO: implement timeout

pub struct Client {
    base_url: String,
    timeout_ms: u64,
}

impl Client {
    pub fn new(base_url: &str) -> Self {
        Client {
            base_url: base_url.to_string(),
            timeout_ms: 5000,
        }
    }

    pub fn deprecated_api(&self, path: &str) -> String {
        format!("{}/{}", self.base_url, path)
    }

    pub fn new_api(&self, path: &str) -> String {
        format!("{}/v2/{}", self.base_url, path)
    }
}
