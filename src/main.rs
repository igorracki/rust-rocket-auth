use std::{collections::{HashMap, HashSet}, sync::Mutex};

use crypto::{sha2::Sha256, digest::Digest};
use lazy_static::lazy_static;
use rocket::{routes, form::Form};
use rocket_dyn_templates::Template;

#[derive(rocket::FromForm)]
struct CreateInfo {
    email: String,
    username: String,
    password: String,
}   

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct User {
    email: String,
    username: String,
    password: String,
}

impl From<CreateInfo> for User {
    fn from(create_info: CreateInfo) -> Self {
        User {
            email: create_info.email,
            username: create_info.username,
            password: create_info.password,
        }
    }
}

lazy_static! {
    static ref USERS: Mutex<HashSet<User>> = Mutex::new(HashSet::new());
}

#[rocket::get("/")]
fn index() -> Template {
    let context = ();
    return Template::render("index", context);
}

#[rocket::post("/register", data="<create_info>")]
fn register(create_info: Form<CreateInfo>) -> Template {
    let user: User = create_info.into_inner().into();
    insert_user(&user);
    let mut context = HashMap::new(); 
    context.insert("email", user.email);
    context.insert("username", user.username);
    return Template::render("welcome", &context);
}

fn insert_user(user: &User) {
    let password_hash = hash_password(&user.password);
    let created_user: User = User {
        password: password_hash,
        ..user.clone()
    };
    if let Ok(mut set) = USERS.lock() {
        set.insert(created_user);
        println!("Successfully inserted user: {:?}", user);
    } else {
        println!("Failed to insert user: {:?}", user);
    }
}

fn hash_password(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(password);
    return hasher.result_str();
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/register", routes![register])
        .attach(Template::fairing())
}
