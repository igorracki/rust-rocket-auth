use std::collections::HashMap;

use rocket_auth::utils;
use rocket_auth::models::User;
use rocket_auth::data::USERS;

#[test]
fn test_insert_user() {
    let user: User = User { username: String::from("test"), email: String::from("test@test.com"), password: String::from("test") };
    let result: HashMap<String, String> = utils::insert_user(&user).expect("Failed to insert user");
    assert_eq!(result["username"], "test");
    assert_eq!(result["email"], "test@test.com");
}

#[test]
#[should_panic(expected = "The email test@test.com is already in use")]
fn test_user_not_added_when_already_exists() {
    reset_users();
    let user: User = User { username: String::from("test"), email: String::from("test@test.com"), password: String::from("test") };
    let _ = utils::insert_user(&user).expect("Failed to insert user");
    let _ = utils::insert_user(&user).unwrap();
}

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

fn reset_users() {
    USERS.write().unwrap().clear();
}
