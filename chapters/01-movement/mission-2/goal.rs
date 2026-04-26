use std::io;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub timeout: u32,
    pub max_connections: usize,
    pub log_level: LogLevel,
    pub data_dir: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            host: "127.0.0.1".to_string(),
            port: 8080,
            timeout: 30,
            max_connections: 100,
            log_level: LogLevel::Info,
            data_dir: PathBuf::from("/tmp/data"),
        }
    }
}

impl Config {
    pub fn with_host(mut self, host: &str) -> Self {
        self.host = host.to_string();
        self
    }

    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn with_timeout(mut self, timeout: u32) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn is_debug(&self) -> bool {
        self.log_level == LogLevel::Debug
    }
}

#[derive(Debug)]
pub struct Server {
    config: Config,
    connections: HashMap<String, u64>,
    running: bool,
}

impl Server {
    pub fn new(config: Config) -> Self {
        Server {
            config,
            connections: HashMap::new(),
            running: false,
        }
    }

    pub fn start(&mut self) -> Result<(), io::Error> {
        if self.running {
            return Err(io::Error::new(io::ErrorKind::Other, "already running"));
        }
        println!(
            "Server starting on {}:{}",
            self.config.host, self.config.port
        );
        self.running = true;
        Ok(())
    }

    pub fn stop(&mut self) {
        if self.running {
            println!("Stopping server...");
            self.connections.clear();
            self.running = false;
        }
    }

    pub fn connection_count(&self) -> usize {
        self.connections.len()
    }

    pub fn add_connection(&mut self, id: String, timestamp: u64) -> bool {
        if self.connections.len() >= self.config.max_connections {
            return false;
        }
        self.connections.insert(id, timestamp);
        true
    }

    pub fn remove_connection(&mut self, id: &str) -> bool {
        self.connections.remove(id).is_some()
    }
}

#[derive(Debug)]
pub struct DataStore {
    path: PathBuf,
    cache: HashMap<String, Vec<u8>>,
}

impl DataStore {
    pub fn new(path: PathBuf) -> Self {
        DataStore {
            path,
            cache: HashMap::new(),
        }
    }

    pub fn load(&mut self, key: &str) -> Result<Vec<u8>, io::Error> {
        if let Some(cached) = self.cache.get(key) {
            return Ok(cached.clone());
        }
        let file_path = self.path.join(key);
        let data = std::fs::read(&file_path)?;
        self.cache.insert(key.to_string(), data.clone());
        Ok(data)
    }

    pub fn store(&mut self, key: &str, data: Vec<u8>) -> Result<(), io::Error> {
        let file_path = self.path.join(key);
        std::fs::write(&file_path, &data)?;
        self.cache.insert(key.to_string(), data);
        Ok(())
    }

    pub fn invalidate(&mut self, key: &str) {
        self.cache.remove(key);
    }
}

pub fn cleanup(server: &mut Server, store: &mut DataStore) -> Result<(), io::Error> {
    store.cache.clear();
    server.stop();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_new() {
        let config = Config::default();
        let server = Server::new(config);
        assert!(!server.running);
        assert_eq!(server.connection_count(), 0);
    }

    #[test]
    fn test_server_start_stop() {
        let config = Config::default();
        let mut server = Server::new(config);
        assert!(server.start().is_ok());
        assert!(server.running);
        server.stop();
        assert!(!server.running);
    }

    #[test]
    fn test_add_connection() {
        let config = Config::default();
        let mut server = Server::new(config);
        let _ = server.start();
        assert!(server.add_connection("conn-1".to_string(), 1000));
        assert_eq!(server.connection_count(), 1);
    }

    #[test]
    fn test_config_builder() {
        let config = Config::default()
            .with_host("0.0.0.0")
            .with_port(9090)
            .with_timeout(60);
        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.port, 9090);
        assert_eq!(config.timeout, 60);
    }
}
