use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String
}

impl User {
    pub fn new(id: i32, username: String, email: String, password: String) -> Self {
        User {
            id,
            username,
            email,
            password
        }
    }
}