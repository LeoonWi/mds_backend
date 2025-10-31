use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    http::StatusCode,
    middleware,
    routing::{delete, get, post},
};

use crate::{
    di::client_container::ClientContainer,
    httpserver::guard::guard,
    models::{employee::Role, request_storage::RequestStorage},
};
use crate::{jwt, models::error::AppError};
use crate::{
    jwt::JWTResponse,
    models::client::{Client, CreateClient, FilterClient, LoginClient},
};

pub fn client_router(container: Arc<ClientContainer>) -> Router {
    Router::new()
        .route("/clients", get(get_clients))
        .route("/clients/{email}", delete(delete_client_by_email))
        .route("/clients/refresh_token", post(refresh_jwt_client))
        .layer(middleware::from_fn(guard))
        .route("/clients/login", post(login_client))
        .route("/signup", post(create_client))
        .with_state(container)
}

async fn create_client(
    State(container): State<Arc<ClientContainer>>,
    Json(payload): Json<CreateClient>,
) -> Result<StatusCode, AppError> {
    container.logic.create_client(payload).await?;
    Ok(StatusCode::CREATED)
}

async fn login_client(
    State(container): State<Arc<ClientContainer>>,
    Json(payload): Json<LoginClient>,
) -> Result<Json<JWTResponse>, AppError> {
    let user_id = container.logic.login(payload).await?;
    let token = jwt::generate(user_id, None)?;
    Ok(Json(token))
}

// PROTECTED

async fn refresh_jwt_client(
    Extension(request_storage): Extension<RequestStorage>,
) -> Result<Json<JWTResponse>, AppError> {
    let token = jwt::generate(request_storage.user_id, None)?;
    Ok(Json(token))
}

async fn get_clients(
    Extension(request_storage): Extension<RequestStorage>,
    State(container): State<Arc<ClientContainer>>,
    Json(filter): Json<FilterClient>,
) -> Result<Json<Vec<Client>>, StatusCode> {
    if request_storage.role < Role::Employee {
        return Err(StatusCode::FORBIDDEN);
    }
    let result: Vec<Client> = container.logic.get_clients(filter).await;
    Ok(Json(result))
}

#[allow(dead_code)]
async fn get_client_by_email(
    State(container): State<Arc<ClientContainer>>,
    Path(email): Path<String>,
    // Extension(role): Extension<Option<Role>>,
) -> Result<Json<Client>, AppError> {
    let result: Result<Json<Client>, AppError> = container.logic.get_client(email).await.map(Json);
    result
}

async fn delete_client_by_email(
    State(container): State<Arc<ClientContainer>>,
    Path(email): Path<String>,
) -> Result<StatusCode, AppError> {
    let result: Result<StatusCode, AppError> =
        container.logic.delete(email).await.map(|_| StatusCode::OK);
    result
}
