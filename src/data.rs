
use std::{sync::Mutex, collections::HashMap};

use lazy_static::lazy_static;

use crate::{models::User, utils::hash_password};

lazy_static! {
    pub static ref USERS: Mutex<HashMap<String, User>> = Mutex::new(HashMap::new());
}

pub fn insert_user(user: &User) -> Result<HashMap<String, String>, String> {
    let mut user_map = USERS.lock().expect("Failed to lock users"); 
    let mut context = HashMap::new();

    if user_map.contains_key(&user.email) {
        return Err(format!("The email {} is already in use", user.email));
    }

    let password_hash = hash_password(&user.password)?;
    let created_user: User = User {
        password: password_hash,
        ..user.clone()
    };
    user_map.insert(user.email.clone(), created_user.clone());
    context.insert("email".to_string(), created_user.email);
    context.insert("username".to_string(), created_user.username);
    return Ok(context);
}

pub fn find_by_username(username: &str) -> Option<User> {
    let user_map = USERS.lock().expect("Failed to lock users");
    return user_map.values().find(|user| user.username == username).map(|user| user.clone());
}

pub fn find_by_email(email: &str) -> Option<User> {
    let user_map = USERS.lock().expect("Failed to lock users");
    return user_map.get(email).map(|user| user.clone());
}
