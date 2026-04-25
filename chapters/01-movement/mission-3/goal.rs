use std::fmt;
use std::io;

#[derive(Debug, Clone)]
pub struct AppConfig {
    /// データベース接続URL
    pub db_url: String,
    pub max_pool_size: u32,
    pub connect_timeout_ms: u64,
    pub idle_timeout_ms: u64,
}

impl AppConfig {
    pub fn new(db_url: &str) -> Self {
        AppConfig {
            db_url: db_url.to_string(),
            max_pool_size: 10,
            connect_timeout_ms: 5000,
            idle_timeout_ms: 600_000,
        }
    }
}

#[derive(Debug)]
pub struct DbClient {
    config: AppConfig,
    connected: bool,
}

impl DbClient {
    pub fn new(config: AppConfig) -> Self {
        DbClient {
            config,
            connected: false,
        }
    }

    pub fn connect(&mut self) -> Result<(), io::Error> {
        println!("Connecting to {}", self.config.db_url.as_str());
        // simulate connection
        if self.config.db_url.is_empty() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "empty URL"));
        }
        self.connected = true;
        Ok(())
    }

    pub fn disconnect(&mut self) {
        if self.connected {
            println!("Disconnecting from {}", self.config.db_url.as_str());
            self.connected = false;
        }
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }

    pub fn execute(&self, query: &str) -> Result<Vec<String>, io::Error> {
        if !self.connected {
            return Err(io::Error::new(io::ErrorKind::NotConnected, "not connected"));
        }
        println!("Executing: {}", query);
        Ok(vec![])
    }
}

pub fn validate(db_url: &str) -> Result<(), String> {
    if db_url.is_empty() || db_url.len() < 10 {
        return Err("db_url must not be empty".to_string());
    }
    if !db_url.starts_with("postgres://") && !db_url.starts_with("mysql://") {
        return Err(format!("unsupported scheme in: {}", db_url));
    }
    Ok(())
}

#[derive(Debug)]
pub enum DbError {
    Connection(String),
    Query(String),
    Validation(String),
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DbError::Connection(msg) => write!(f, "connection error: {}", msg),
            DbError::Query(msg)      => write!(f, "query error: {}", msg),
            DbError::Validation(msg) => write!(f, "validation error: {}", msg),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_empty() {
        assert!(validate("").is_err());
    }

    #[test]
    fn test_validate_short_url() {
        assert!(validate("pg://a").is_err());
    }

    #[test]
    fn test_validate_valid_postgres() {
        assert!(validate("postgres://localhost/mydb").is_ok());
    }

    #[test]
    fn test_validate_invalid_scheme() {
        assert!(validate("http://localhost/mydb").is_err());
    }

    #[test]
    fn test_connect_empty_url() {
        let config = AppConfig::new("");
        let mut client = DbClient::new(config);
        assert!(client.connect().is_err());
    }

    #[test]
    fn test_connect_success() {
        let config = AppConfig::new("postgres://localhost/test");
        let mut client = DbClient::new(config);
        assert!(client.connect().is_ok());
        assert!(client.is_connected());
    }
}
