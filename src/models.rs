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

impl Into<User> for UserRegistration {
    fn into(self) -> User {
        User {
            email: self.email,
            username: self.username,
            password: self.password,
        }
    }
}

