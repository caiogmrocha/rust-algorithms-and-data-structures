use crate::domain::entities::user::User;

pub trait UsersRepository: Sync + Send {
    fn get_all(&self) -> Vec<User>;
    fn get_by_id(&self, id: i32) -> Option<User>;
    fn create(&self, user: User) -> Result<(), String>;
    fn update(&self, user: User) -> Result<(), String>;
}