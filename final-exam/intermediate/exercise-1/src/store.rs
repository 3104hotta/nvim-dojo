//! store module
use log::error;

pub fn save(key: &str, value: &[u8]) -> Result<(), String> {
    if key.is_empty() {
        log::error!("empty key");
        return Err("empty key".to_string());
    }
    log::info!("save key={} len={}", key, value.len());
    Ok(())
}

pub fn load(key: &str) -> Option<Vec<u8>> {
    log::info!("load {}", key);
    None
}
