use std::sync::Arc;

use crate::{app::contracts::users_repository::UsersRepository, domain::entities::user::User};

pub struct GetUserByIdService {
    users_repository: Arc<dyn UsersRepository>,
}

impl GetUserByIdService {
    pub fn new(users_repository: Arc<dyn UsersRepository>) -> Self {
        Self {
            users_repository,
        }
    }
    
    pub async fn execute(&self, id: i32) -> Option<User> {
        self.users_repository.get_by_id(id)
    }
}