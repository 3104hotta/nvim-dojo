use crate::config::Config;

// TODO: add TLS support
// TODO: add metrics

#[derive(Debug)]
pub struct Server {
    host: String,
    port: u16,
}

impl Server {
    pub fn new(cfg: &Config) -> Self {
        Server {
            host: cfg.host.clone(),
            port: cfg.port,
        }
    }

    pub fn deprecated_api(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn new_api(&self) -> String {
        format!("http://{}:{}", self.host, self.port)
    }
}
