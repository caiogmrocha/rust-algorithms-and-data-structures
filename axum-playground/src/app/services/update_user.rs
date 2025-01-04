use std::sync::Arc;

use crate::{
    app::contracts::users_repository::UsersRepository,
    domain::entities::user::User,
};

pub struct UpdateUserService {
    users_repository: Arc<dyn UsersRepository>,
}

impl UpdateUserService {
    pub fn new(users_repository: Arc<dyn UsersRepository>) -> Self {
        Self {
            users_repository,
        }
    }

    pub async fn execute(&self, user: User) -> Result<(), String> {
        self.users_repository.update(user)
    }
}

