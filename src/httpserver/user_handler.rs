use std::sync::Arc;

use axum::Json;
use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Router, http::StatusCode};
use serde::Deserialize;

use crate::di::user_container::UserContainer;
use crate::models::user::User;
use crate::models::error::AppError;

pub fn user_router(container: Arc<UserContainer>) -> Router {
    Router::new()
        .route("/user", post(create_user))
        .route("/user", get(get_users))
        .route("/user/email/{email}", get(get_user_by_email))
        .route("/user/phone/{phone}", get(get_user_by_phone))
        .with_state(container)
}

#[derive(Deserialize)]
struct CreateUser {
    pub name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub email: String,
    pub phone: String,
    pub password: String,
    pub inn: Option<String>,
    pub snils: Option<String>,
}

async fn create_user(
    State(container): State<Arc<UserContainer>>,
    Json(payload): Json<CreateUser>,
) -> Result<StatusCode, AppError> {
    container
        .logic
        .create_user(
            payload.name,
            payload.last_name,
            payload.middle_name,
            payload.email,
            payload.phone,
            payload.password,
            payload.inn,
            payload.snils,
        )
        .await?;

    Ok(StatusCode::OK)
}

async fn get_users(State(container): State<Arc<UserContainer>>) -> Json<Vec<User>> {
    Json(container.logic.get_users().await)
}

async fn get_user_by_email(
    State(container): State<Arc<UserContainer>>,
    Path(email): Path<String>,
) -> Result<Json<User>, AppError> {
    container.logic.get_user_by_email(email).await.map(Json)
}

async fn get_user_by_phone(
    State(container): State<Arc<UserContainer>>,
    Path(phone): Path<String>,
) -> Result<Json<User>, AppError> {
    container.logic.get_user_by_phone(phone).await.map(Json)
}