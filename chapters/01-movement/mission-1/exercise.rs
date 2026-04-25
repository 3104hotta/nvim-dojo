use std::fs;
use std::io;

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub timeout: u64,
    pub debug: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            host: "localhost".to_string(),
            port: 8080,
            timeout: 30,
            debug: false,
        }
    }
}

pub fn parse_config(path: &str) -> Result<Config, io::Error> {
    // TODO: validate
    let content = fs::read_to_string(path)?;
    let mut config = Config::default();

    for line in content.lines() {
        let parts: Vec<&str> = line.splitn(2, '=').collect();
        if parts.len() != 2 {
            continue;
        }
        let key = parts[0].trim();
        let value = parts[1].trim();

        match key {
            "host"    => config.host = value.to_string(),
            "port"    => config.port = value.parse().unwrap_or(8080),
            "timeout" => config.timeout = value.parse().unwrap_or(30),
            "debug"   => config.debug = value == "true",
            _         => {}
        }
    }

    Ok(config)
}

pub fn connect(config: &Config) -> Result<(), io::Error> {
    let timeout = 30;
    let addr = format!("{}:{}", config.host, config.port);
    println!("Connecting to {} (timeout={}s)", addr, timeout);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_host() {
        let config = Config::default();
        assert_eq!(config.host, "localhost");
    }

    #[test]
    fn test_default_timeout() {
        let config = Config::default();
        assert_eq!(config.timeout, 30);
    }

    #[test]
    fn test_connect_addr() {
        let config = Config {
            host: "localhost".to_string(),
            port: 9090,
            timeout: 30,
            debug: false,
        };
        assert!(connect(&config).is_ok());
    }
}
