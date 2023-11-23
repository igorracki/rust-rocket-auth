use rocket_auth::{utils::{self, generate_token}, models::Claim};

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

#[test]
fn test_generate_token() {
    let username: String = String::from("test");
    let token: String = generate_token(&username).expect("Failed to generate token");
    assert_ne!(token, "");
}

#[test]
fn test_validate_token() {
    let username: String = String::from("test");
    let token: String = generate_token(&username).expect("Failed to generate token");
    let result: Result<Claim, String> = utils::validate_token(&token);
    match result {
        Ok(claim) => assert_eq!(claim.sub, username),
        Err(e) => panic!("{}", e) 
    };
}

#[test]
fn test_validate_wrong_token() {
    let token: &'static str = "wrong_token";
    let result: Result<Claim, String> = utils::validate_token(&token);
    assert_eq!(result.is_ok(), false);
}
