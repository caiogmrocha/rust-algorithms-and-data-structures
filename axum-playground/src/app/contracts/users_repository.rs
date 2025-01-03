use crate::domain::entities::user::User;

pub trait UsersRepository: Sync + Send {
    fn create(&self, user: User) -> Result<(), String>;
}