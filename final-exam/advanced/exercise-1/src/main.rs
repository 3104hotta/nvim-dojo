// OSS-style mid-size codebase to refactor.
// Goals (see README.md for details):
//   - Migrate Error from thiserror-style to a custom AppError enum
//   - Standardise async ? + .map_err handling
//   - Remove or implement #[allow(dead_code)] functions
//   - Reorder impl blocks: public API → private → trait impls
//   - cargo fmt + cargo clippy

use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    NotFound,
    Invalid(String),
    Io(std::io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NotFound       => write!(f, "not found"),
            Error::Invalid(m)     => write!(f, "invalid: {}", m),
            Error::Io(e)          => write!(f, "io: {}", e),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self { Error::Io(e) }
}

#[derive(Debug, Clone)]
pub struct Item {
    pub id: u64,
    pub name: String,
}

pub struct Store {
    items: HashMap<u64, Item>,
}

impl Store {
    fn key_for(id: u64) -> String { format!("item:{}", id) }

    #[allow(dead_code)]
    fn legacy_dump(&self) -> Vec<String> {
        self.items.values().map(|i| i.name.clone()).collect()
    }

    pub fn new() -> Self { Store { items: HashMap::new() } }

    pub async fn fetch(&self, id: u64) -> Result<Item, Error> {
        let raw = std::fs::read_to_string(format!("/tmp/{}", Self::key_for(id)))
            .map_err(Error::Io)?;
        Ok(Item { id, name: raw.trim().to_string() })
    }

    #[allow(dead_code)]
    fn unused_helper(&self) -> usize { self.items.len() }

    pub fn add(&mut self, item: Item) {
        self.items.insert(item.id, item);
    }
}

impl Default for Store {
    fn default() -> Self { Self::new() }
}

#[allow(dead_code)]
fn orphan_helper() -> &'static str { "unused" }

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut store = Store::new();
    store.add(Item { id: 1, name: "alpha".to_string() });
    let _ = store.fetch(1).await;
    Ok(())
}
