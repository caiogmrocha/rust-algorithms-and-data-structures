use crate::{
    app::contracts::users_repository::UsersRepository,
    config::database::Database,
    domain::entities::user::User,
};

pub struct InMemoryUsersRepository {
    db: Database<User>
}

impl UsersRepository for InMemoryUsersRepository {
    fn get_all(&self) -> Vec<User> {
        let db = self.db.lock().unwrap();

        db.values().cloned().collect()
    }

    fn create(&self, user: User) -> Result<(), String> {
        let mut db = self.db.lock().unwrap();
        let id = db.len() as i32 + 1;

        db.insert(id.to_string(), user);

        Ok(())
    }
}

impl InMemoryUsersRepository {
    pub fn new(db: Database<User>) -> Self {
        Self {
            db
        }
    }
}