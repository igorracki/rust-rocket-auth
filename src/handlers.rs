use std::collections::HashMap;

use rocket::{form::Form, http::{Cookie, SameSite, CookieJar}};
use rocket_dyn_templates::Template;

use crate::{models::{UserRegistration, User, UserLogin, JWT}, utils::{verify_password, self}, data::{insert_user, find_by_email}};

#[rocket::get("/")]
pub fn index() -> Template {
    let context = ();
    return Template::render("index", context);
}

#[rocket::post("/register", data="<user_registration>")]
pub fn register(user_registration: Form<UserRegistration>) -> Template {
    let user: User = user_registration.into_inner().into();
    return match insert_user(&user) {
        Ok(context) => Template::render("welcome", &context),
        Err(error) => {
            let mut context = HashMap::new();
            context.insert("error".to_string(), error);
            return Template::render("registrationPanel", &context);
        },
    };
}


#[rocket::post("/login", data="<user_login>")]
pub fn login(user_login: Form<UserLogin>, cookies: &CookieJar<'_>) -> Template {
    let mut context = HashMap::new();
    let template: &str;
    let user_login_info: UserLogin = user_login.into_inner();

    if let Some(user) = find_by_email(&user_login_info.email) {
        if verify_password(&user_login_info.password, &user.password) {
            match utils::generate_token(&user.username) {
                Ok(token) => {
                    let jwt_cookie = Cookie::build(("jwt", token)).same_site(SameSite::Strict);
                    cookies.add(jwt_cookie);
                },
                Err(error) => { // TODO: Improve this...
                    context.insert("error".to_string(), "Something went wrong, please try again".to_string());
                    println!("Error while generating token: {}", error);
                    return Template::render("loginPanel", &context);
                },
            };
            context.insert("username".to_string(), user.username.clone());
            template = "userArea";
        } else {
            context.insert("error".to_string(), "Invalid password".to_string());
            template = "loginPanel"; 
        }
    } else {
        context.insert("error".to_string(), "Invalid email".to_string());
        template = "loginPanel"; 
    }
    return Template::render(template, &context);
}

#[rocket::get("/dashboard")]
pub fn dashboard(jwt: JWT) -> Template {
    let mut context = HashMap::new();
    context.insert("username".to_string(), jwt.claim.sub);
    return Template::render("userArea", &context);
}

