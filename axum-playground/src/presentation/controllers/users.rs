use std::sync::Arc;

use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use crate::{
    app::services::{
        create_user::CreateUserService,
        get_all_users::GetAllUsersService,
        get_user_by_id::GetUserByIdService,
    },
    config::database::DB,
    domain::entities::user::User,
    infra::repositories::users_repository::InMemoryUsersRepository,
};

pub struct UsersController {
    create_user_service: Arc<CreateUserService>,
    get_all_users_service: Arc<GetAllUsersService>,
    get_user_by_id_service: Arc<GetUserByIdService>,
}

impl UsersController {
    pub fn new() -> Self {
        let db = DB.clone();
        let users_repository = Arc::new(InMemoryUsersRepository::new(db));
        let create_user_service = Arc::new(CreateUserService::new(users_repository.clone()));
        let get_all_users_service = Arc::new(GetAllUsersService::new(users_repository.clone()));
        let get_user_by_id_service = Arc::new(GetUserByIdService::new(users_repository.clone()));

        Self {
            create_user_service: create_user_service.clone(),
            get_all_users_service: get_all_users_service.clone(),
            get_user_by_id_service: get_user_by_id_service.clone(),
        }
    }

    pub async fn get_all(State(this): State<Arc<Self>>) -> impl IntoResponse {
        let users = this.get_all_users_service.get_all().await;

        (StatusCode::OK, Json(users).into_response())
    }

    pub async fn get_by_id(
        State(this): State<Arc<UsersController>>,
        Path(id): Path<i32>,
    ) -> impl IntoResponse {
        match this.get_user_by_id_service.execute(id).await {
            Some(user) => (StatusCode::OK, Json(user).into_response()),
            None => (StatusCode::NOT_FOUND, Json(json!({ "message": "User not found" })).into_response()),
        }
    }

    pub async fn create(
        State(this): State<Arc<Self>>,
        Json(user): Json<User>,
    ) -> impl IntoResponse {
        println!("{:?}", user);

        this.create_user_service.create(user).await.unwrap();

        (StatusCode::CREATED, Json(json!({ "message": "User created" })).into_response())
    }
}