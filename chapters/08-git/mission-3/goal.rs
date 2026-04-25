pub struct Validator {
    name: String,
}

impl Validator {
    pub fn new(name: &str) -> Self {
        Validator { name: name.to_string() }
    }

    pub fn validate(&self, input: &str, strict: bool) -> Result<(), String> {
        if input.is_empty() {
            return Err(format!("{}: empty input", self.name));
        }
        if strict && input.len() >= 1024 {
            return Err(format!("{}: too long", self.name));
        }
        Ok(())
    }
}
