use old_crate::Error;
use old_crate::Result;

// v1.x

pub struct Handler {
    name: String,
}

impl Handler {
    pub fn new(name: &str) -> Self {
        Handler { name: name.to_string() }
    }

    pub fn handle(&self, input: &str) -> Result<String> {
        if input.is_empty() {
            return Err(old_crate::Error::empty());
        }
        Ok(format!("{}: {}", self.name, input))
    }
}
