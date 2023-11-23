use std::collections::HashMap;

use rocket_auth::data::{self, USERS};

use rocket_auth::models::User;

#[allow(dead_code)]
fn reset_users() {
    // TODO: very weird race condition here...
    USERS.lock().unwrap().clear();
}

#[test]
fn test_insert_user() {
    let user: User = User { username: String::from("test"), email: String::from("test@test.com"), password: String::from("test") };
    let result: HashMap<String, String> = data::insert_user(&user).expect("Failed to insert user");
    assert_eq!(result["username"], "test");
    assert_eq!(result["email"], "test@test.com");
}

#[test]
#[should_panic(expected = "The email test1@test1.com is already in use")]
fn test_user_not_added_when_already_exists() {
    let user: User = User { username: String::from("test1"), email: String::from("test1@test1.com"), password: String::from("test1") };
    let _ = data::insert_user(&user).expect("Failed to insert user");
    let _ = data::insert_user(&user).unwrap();
}

#[test]
fn test_find_by_username() {
    let user: User = User { username: String::from("test2"), email: String::from("test2@test2.com"), password: String::from("test2") };
    let _ = data::insert_user(&user).expect("Failed to insert user");
    let result: Option<User> = data::find_by_username("test2");
    let found_user: User = result.expect("Failed to find user");
    assert_eq!(found_user.username, "test2");
    assert_eq!(found_user.email, "test2@test2.com");
}

#[test]
fn test_find_by_email() {
    let user: User = User { username: String::from("test3"), email: String::from("test3@test3.com"), password: String::from("test3") };
    let _ = data::insert_user(&user).expect("Failed to insert user");
    let result: Option<User> = data::find_by_email("test3@test3.com");
    let found_user: User = result.expect("Failed to find user");
    assert_eq!(found_user.email, "test3@test3.com");
    assert_eq!(found_user.username, "test3");
}
