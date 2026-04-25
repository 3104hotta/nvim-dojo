#[derive(Debug, Clone)]
pub struct User {
    pub id: u64,
    pub name: String,
    // Add: pub created_at: chrono::DateTime<chrono::Utc>,
}

impl User {
    pub fn new(id: u64, name: &str) -> Self {
        User { id, name: name.to_string() }
    }
}
