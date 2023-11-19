use std::collections::HashMap;

use bcrypt::{hash, verify};

use crate::{models::User, data::USERS};

pub fn insert_user(user: &User) -> Result<HashMap<String, String>, String> {
    let mut user_map = USERS.write().map_err(|e| e.to_string())?;
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

pub fn hash_password(password: &str) -> Result<String, String> {
    return hash(password, 12).map_err(|e| e.to_string());
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    match verify(password, hash) {
        Ok(result) => return result,
        Err(_) => return false,
    };
}


