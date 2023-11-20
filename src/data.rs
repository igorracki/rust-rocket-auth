
use std::{sync::RwLock, collections::HashMap};

use lazy_static::lazy_static;

use crate::models::User;

lazy_static! {
    pub static ref USERS: RwLock<HashMap<String, User>> = RwLock::new(HashMap::new());
}

