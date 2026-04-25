//! middleware module
use log::warn;

pub fn auth(token: &str) -> bool {
    if token.is_empty() {
        log::warn!("empty token");
        return false;
    }
    log::info!("token ok");
    true
}
