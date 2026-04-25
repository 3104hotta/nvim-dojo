use crate::db;
use crate::model::User;

pub fn get_user(id: u64) -> Option<String> {
    let user = db::find_user(id)?;
    Some(format!(r#"{{"id":{},"name":"{}"}}"#, user.id, user.name))
}

pub fn create_user(name: &str) -> Result<String, String> {
    let user = User { id: 0, name: name.to_string() };
    db::insert_user(user.clone())?;
    Ok(format!(r#"{{"id":{},"name":"{}"}}"#, user.id, user.name))
}
