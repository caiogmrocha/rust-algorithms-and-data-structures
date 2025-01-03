use std::sync::Arc;

use crate::{
    app::contracts::users_repository::UsersRepository,
    domain::entities::user::User,
};

pub struct GetAllUsersService {
    users_repository: Arc<dyn UsersRepository>,
}

impl GetAllUsersService {
    pub fn new(users_repository: Arc<dyn UsersRepository>) -> Self {
        Self {
            users_repository,
        }
    }

    pub async fn get_all(&self) -> Vec<User> {
        self.users_repository.get_all()
    }
}