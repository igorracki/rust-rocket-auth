use rocket::{request::{FromRequest, Outcome}, Request};
use serde::{Deserialize, Serialize};

use crate::utils;

#[derive(rocket::FromForm, Serialize)]
pub struct UserRegistration {
    pub email: String,
    pub username: String,
    pub password: String,  
}  

#[derive(rocket::FromForm, Serialize)]
pub struct UserLogin {
    pub email: String,
    pub password: String,
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct User {
    pub email: String,
    pub username: String,
    pub password: String,
}

impl Into<User> for UserRegistration {
    fn into(self) -> User {
        User {
            email: self.email,
            username: self.username,
            password: self.password,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Claim {
    pub sub: String,
    pub exp: usize,
    pub role: String,
}

pub struct JWT {
    pub claim: Claim,
}

#[rocket::async_trait]
impl<'r> FromRequest <'r> for JWT {
    type Error = String;
    
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, String> {
       match request.headers().get_one("Authorization") {
           Some(token) => {
               match utils::validate_token(token) {
                   Ok(claim) => return Outcome::Success(JWT { claim }),
                   Err(e) => return Outcome::Error((rocket::http::Status::Unauthorized, e)),
               };
           },
           None => return Outcome::Error((rocket::http::Status::Unauthorized, "Missing token".to_owned())),
       }; 
    }
}

