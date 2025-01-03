use crate::domain::entities::user::User;

pub trait UsersRepository: Sync + Send {
    fn get_all(&self) -> Vec<User>;
    fn create(&self, user: User) -> Result<(), String>;
}