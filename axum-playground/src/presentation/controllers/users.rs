use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};

use crate::{
    app::services::create_user_service::CreateUserService,
    config::database::DB,
    domain::entities::user::User,
    infra::repositories::users_repository::InMemoryUsersRepository,
};

pub struct UsersController {
    create_user_service: Arc<CreateUserService>
}

impl UsersController {
    pub fn new() -> Self {
        let db = DB.clone();
        let users_repository = InMemoryUsersRepository::new(db);
        let create_user_service = CreateUserService::new(Arc::new(users_repository));

        Self {
            create_user_service: Arc::new(create_user_service)
        }
    }

    pub async fn create(
        State(this): State<Arc<Self>>,
        Json(user): Json<User>,
    ) -> impl IntoResponse {
        println!("{:?}", user);

        this.create_user_service.create(user).await.unwrap();

        "User created".into_response()
    }
}