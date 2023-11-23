use bcrypt::{hash, verify};

pub fn hash_password(password: &str) -> Result<String, String> {
    return hash(password, 12).map_err(|e| e.to_string());
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    match verify(password, hash) {
        Ok(result) => return result,
        Err(_) => return false,
    };
}


