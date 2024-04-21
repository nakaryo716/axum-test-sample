use axum::{response::IntoResponse, Json};

use crate::user::{CreateUser, User};

pub async fn root() -> impl IntoResponse {
    "Hello world".to_string()
}

pub async fn create_user(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    let res = User::new(payload);

    Json(res)
}
