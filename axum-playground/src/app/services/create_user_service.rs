use std::sync::Arc;

use crate::{app::contracts::users_repository::UsersRepository, domain::entities::user::User};

pub struct CreateUserService {
    users_repository: Arc<dyn UsersRepository>
}

impl CreateUserService {
    pub fn new(users_repository: Arc<dyn UsersRepository>) -> Self {
        Self {
            users_repository
        }
    }

    pub async fn create(&self, user: User) -> Result<(), String> {
        self.users_repository.create(user)
    }
}