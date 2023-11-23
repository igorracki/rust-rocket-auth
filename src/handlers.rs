use std::collections::HashMap;

use rocket::form::Form;
use rocket_dyn_templates::Template;

use crate::{models::{UserRegistration, User, UserLogin}, utils::verify_password, data::{insert_user, find_by_email}};

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
pub fn login(user_login: Form<UserLogin>) -> Template {
    let mut context = HashMap::new();
    let user_login_info: UserLogin = user_login.into_inner();

    if let Some(user) = find_by_email(&user_login_info.email) {
        if verify_password(&user_login_info.password, &user.password) {
            context.insert("username".to_string(), user.username.clone());
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

