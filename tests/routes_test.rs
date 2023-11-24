use rocket::form::Form;
use rocket::http::{Status, ContentType, Cookie};
use rocket::local::blocking::Client;
use rocket_auth::models::{UserRegistration, UserLogin};
use rocket_auth::server;

#[test]
fn test_index() {
    let rocket = server::server(); 
    let client = Client::tracked(rocket).unwrap();
    let request = client.get("/");
    let response = request.dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert!(response.into_string().unwrap().contains("<title>Home</title>"));
}

#[test]
fn test_register() {
    let rocket = server::server(); 
    let client = Client::tracked(rocket).unwrap();
    let user_registration = UserRegistration {
        email: String::from("test@test.com"),
        username: String::from("test"),
        password: String::from("test"),
    };
    let registration_form = serde_urlencoded::to_string(&user_registration).unwrap();
    let request = client.post("/register")
        .header(ContentType::Form)
        .body(registration_form);
    let response = request.dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert!(response.into_string().unwrap().contains("Welcome test"));
}

#[test]
fn test_login() {
    let rocket = server::server(); 
    let client = Client::tracked(rocket).unwrap();
    let user_registration = UserRegistration {
        email: String::from("test1@test1.com"),
        username: String::from("test1"),
        password: String::from("test1"),
    };
    let registration_form = serde_urlencoded::to_string(&user_registration).unwrap();
    client.post("/register")
        .header(ContentType::Form)
        .body(registration_form)
        .dispatch();
    
    let user_login = UserLogin {
        email: String::from("test1@test1.com"),
        password: String::from("test1"),
    };
    let login_form = serde_urlencoded::to_string(&user_login).unwrap();
    let login_response = client.post("/login")
        .header(ContentType::Form)
        .body(login_form)
        .dispatch();
    assert_eq!(login_response.status(), Status::Ok);
    let jwt_cookie = login_response.cookies().get("jwt");
    assert!(jwt_cookie.is_some());

    let authorized_request = client.get("/dashboard")
        .header(rocket::http::Header::new("Authorization", jwt_cookie.unwrap().value().to_string()))
        .dispatch();
    assert_eq!(authorized_request.status(), Status::Ok);
}

#[test]
fn test_auth() {
    let rocket = server::server(); 
    let client = Client::tracked(rocket).unwrap();
    let request = client.get("/dashboard");
    let response = request.dispatch();
    assert_eq!(response.status(), Status::Unauthorized);
}
