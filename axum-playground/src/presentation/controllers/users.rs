use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};

use crate::{
    app::services::{
        create_user::CreateUserService,
        get_all_users::GetAllUsersService,
    },
    config::database::DB,
    domain::entities::user::User,
    infra::repositories::users_repository::InMemoryUsersRepository,
};

pub struct UsersController {
    create_user_service: Arc<CreateUserService>,
    get_all_users_service: Arc<GetAllUsersService>,
}

impl UsersController {
    pub fn new() -> Self {
        let db = DB.clone();
        let users_repository = Arc::new(InMemoryUsersRepository::new(db));
        let create_user_service = Arc::new(CreateUserService::new(users_repository.clone()));
        let get_all_users_service = Arc::new(GetAllUsersService::new(users_repository.clone()));

        Self {
            create_user_service: create_user_service.clone(),
            get_all_users_service: get_all_users_service.clone(),
        }
    }

    pub async fn get_all(State(this): State<Arc<Self>>) -> impl IntoResponse {
        let users = this.get_all_users_service.get_all().await;

        Json(users).into_response()
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