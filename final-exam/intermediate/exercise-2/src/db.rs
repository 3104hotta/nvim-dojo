use crate::model::User;

// Adjust signature when User gains `created_at`.
pub fn insert_user(user: User) -> Result<(), String> {
    println!("INSERT users (id={}, name={})", user.id, user.name);
    Ok(())
}

pub fn find_user(id: u64) -> Option<User> {
    if id == 0 {
        return None;
    }
    Some(User { id, name: format!("user-{}", id) })
}
