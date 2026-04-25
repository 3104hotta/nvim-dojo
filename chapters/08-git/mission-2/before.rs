use std::io;

// Outdated comment about behaviour.
// Old style boilerplate.

pub struct Repo {
    path: String,
}

impl Repo {
    pub fn new(path: &str) -> Self {
        Repo { path: path.to_string() }
    }

    pub fn open(&self) -> Result<String, io::Error> {
        let s = std::fs::read_to_string(&self.path).unwrap();
        Ok(s)
    }

    pub fn write(&self, content: &str) {
        std::fs::write(&self.path, content).unwrap();
    }
}

pub fn detect(path: &str) -> bool {
    std::path::Path::new(path).exists()
}
