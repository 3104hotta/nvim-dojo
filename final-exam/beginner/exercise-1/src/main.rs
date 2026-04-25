use std::collections::HashMap;
use std::io;

#[derive(Debug, Default)]
pub struct Config {
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn default() -> Self {
        Config {
            host: "127.0.0.1".to_string(),
            port: 8080,
        }
    }

    pub fn load() -> Result<Self, io::Error> {
        Ok(Self::default())
    }
}

fn main() {
    let config = Config::default();
    println!("server starting on {}:{}", config.host, config.port);
}
