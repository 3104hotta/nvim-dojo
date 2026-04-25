// Run `setup.sh` once to recreate the merge conflict.
// After resolving, `git status` should be clean.

pub struct Validator {
    name: String,
}

impl Validator {
    pub fn new(name: &str) -> Self {
        Validator { name: name.to_string() }
    }

<<<<<<< HEAD
    pub fn validate(&self, input: &str, strict: bool) -> bool {
        if strict {
            !input.is_empty() && input.len() < 1024
        } else {
            !input.is_empty()
        }
    }
=======
    pub fn validate(&self, input: &str) -> Result<(), String> {
        if input.is_empty() {
            return Err(format!("{}: empty input", self.name));
        }
        if input.len() >= 1024 {
            return Err(format!("{}: too long", self.name));
        }
        Ok(())
    }
>>>>>>> feature
}
