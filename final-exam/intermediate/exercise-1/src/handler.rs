//! handler module
use log::{error, info};

pub fn handle(req: &str) -> Result<String, String> {
    log::info!("handling: {}", req);
    if req.is_empty() {
        log::error!("empty request");
        return Err("empty".to_string());
    }
    Ok(format!("ok:{}", req))
}
