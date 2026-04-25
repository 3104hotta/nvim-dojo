// TODO: support TOML config file

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub debug: bool,
}

pub fn load() -> Config {
    Config {
        host: "127.0.0.1".to_string(),
        port: 8080,
        debug: false,
    }
}

pub fn deprecated_api() -> &'static str {
    "v1"
}

pub fn new_api() -> &'static str {
    "v2"
}
