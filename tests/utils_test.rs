use rocket_auth::utils;

#[test]
fn test_hash_password() {
    let password: String = String::from("test");
    let hashed_password: String = utils::hash_password(&password).expect("Failed to hash password");
    assert_ne!(password, hashed_password);
}

#[test]
fn test_verify_password() {
    let password: String = String::from("test");
    let hashed_password: String = utils::hash_password(&password).expect("Failed to hash password");
    let result: bool = utils::verify_password(&password, &hashed_password);
    assert_eq!(result, true);
}

#[test]
fn test_verify_password_invalid() {
    let password: String = String::from("test");
    let hashed_password: String = utils::hash_password(&password).expect("Failed to hash password");
    let result: bool = utils::verify_password(&String::from("invalid"), &hashed_password);
    assert_eq!(result, false);
}

