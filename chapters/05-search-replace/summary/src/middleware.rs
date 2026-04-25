use old_crate::Error;

// v1.x

pub fn auth(token: &str) -> Result<(), old_crate::Error> {
    if token.is_empty() {
        return Err(old_crate::Error::unauthorized());
    }
    Ok(())
}

pub fn rate_limit(key: &str, limit: u32) -> Result<(), old_crate::Error> {
    let _ = (key, limit);
    Ok(())
}
