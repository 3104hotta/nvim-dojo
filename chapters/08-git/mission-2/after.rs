use std::io;

pub struct Repo {
    path: String,
}

impl Repo {
    pub fn new(path: &str) -> Self {
        Repo { path: path.to_string() }
    }

    pub fn open(&self) -> Result<String, io::Error> {
        std::fs::read_to_string(&self.path)
    }

    pub fn write(&self, content: &str) -> Result<(), io::Error> {
        std::fs::write(&self.path, content)
    }
}

pub fn detect(path: &str) -> bool {
    std::path::Path::new(path).exists()
}

pub fn list_dir(path: &str) -> Result<Vec<String>, io::Error> {
    let entries = std::fs::read_dir(path)?
        .filter_map(|e| e.ok())
        .filter_map(|e| e.file_name().into_string().ok())
        .collect();
    Ok(entries)
}
