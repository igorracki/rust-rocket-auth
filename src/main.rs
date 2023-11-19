use std::{collections::HashMap, sync::Mutex};

use bcrypt::{hash, verify};
use lazy_static::lazy_static;
use rocket::{routes, form::Form};
use rocket_dyn_templates::Template;

#[derive(rocket::FromForm)]
struct UserRegistration {
    email: String,
    username: String,
    password: String,  
}  

#[derive(rocket::FromForm)]
struct UserLogin {
    email: String,
    password: String,
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct User {
    email: String,
    username: String,
    password: String,
}

impl From<UserRegistration> for User {
    fn from(user_registration: UserRegistration) -> Self {
        User {
            email: user_registration.email,
            username: user_registration.username,
            password: user_registration.password,
        }
    }
}

lazy_static! {
    static ref USERS: Mutex<HashMap<String, User>> = Mutex::new(HashMap::new());
}

#[rocket::get("/")]
fn index() -> Template {
    let context = ();
    return Template::render("index", context);
}

#[rocket::post("/register", data="<user_registration>")]
fn register(user_registration: Form<UserRegistration>) -> Template {
    println!("Called");
    let user: User = user_registration.into_inner().into();
    let context = insert_user(&user);
    return Template::render("welcome", &context);
}

fn insert_user(user: &User) -> HashMap<String, String> {
    let mut user_map = USERS.lock().unwrap();
    let mut context = HashMap::new();
    if user_map.contains_key(&user.email) {
        println!("User already exists: {:?}", user.email);
        context.insert("error".to_string(), format!("The email {} is already in use", user.email));
        return context;
    }

    let password_hash = hash_password(&user.password);
    let created_user: User = User {
        password: password_hash,
        ..user.clone()
    };
    user_map.insert(user.email.clone(), created_user.clone());
    println!("Successfully inserted user: {:?}", created_user);

    context.insert("email".to_string(), created_user.email);
    context.insert("username".to_string(), created_user.username);
    return context;
}

fn hash_password(password: &str) -> String {
    let hashed_password = hash(password, 12).unwrap();
    return hashed_password;
}

#[rocket::post("/login", data="<user_login>")]
fn login(user_login: Form<UserLogin>) -> Template {
    let user_map = USERS.lock().unwrap();
    let mut context = HashMap::new();
    let user_login_info: UserLogin = user_login.into_inner();
    if user_map.contains_key(&user_login_info.email) {
        let user: User = user_map.get(&user_login_info.email).unwrap().clone();
        if verify_password(&user_login_info.password, &user.password) {
            context.insert("username".to_string(), user.username);
            return Template::render("userArea", &context);
        } else {
            context.insert("error".to_string(), "Invalid password".to_string());
            return Template::render("loginPanel", &context);
        }
    } else {
        context.insert("error".to_string(), "Invalid email".to_string());
        return Template::render("loginPanel", &context);
    }
}

fn verify_password(password: &str, hash: &str) -> bool {
    match verify(password, hash) {
        Ok(result) => return result,
        Err(_) => return false,
    };
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![register])
        .mount("/", routes![login])
        .attach(Template::fairing())
}
