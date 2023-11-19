
use std::{sync::RwLock, collections::HashMap};

use crate::{lazy_static, models::User};

lazy_static! {
    pub static ref USERS: RwLock<HashMap<String, User>> = RwLock::new(HashMap::new());
}

