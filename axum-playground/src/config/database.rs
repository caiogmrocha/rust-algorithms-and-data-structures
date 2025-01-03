use std::{
    collections::HashMap, sync::{Arc, LazyLock, Mutex}
};

use crate::domain::entities::user::User;

pub type Database<T> = Arc<Mutex<HashMap<String, T>>>;

pub static DB: LazyLock<Database<User>> = LazyLock::new(|| Arc::new(Mutex::new(HashMap::new())));