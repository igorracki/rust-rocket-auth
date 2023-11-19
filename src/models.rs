#[derive(rocket::FromForm)]
pub struct UserRegistration {
    pub email: String,
    pub username: String,
    pub password: String,  
}  

#[derive(rocket::FromForm)]
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

impl From<UserRegistration> for User {
    fn from(user_registration: UserRegistration) -> Self {
        User {
            email: user_registration.email,
            username: user_registration.username,
            password: user_registration.password,
        }
    }
}

