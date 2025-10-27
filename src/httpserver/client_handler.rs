use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post},
};

use crate::di::client_container::ClientContainer;
use crate::models::client::{Client, CreateClient, FilterClient};
use crate::models::error::AppError;

pub fn client_router(container: Arc<ClientContainer>) -> Router {
    Router::new()
        .route("/signup", post(create_client))
        .route("/clients", get(get_clients))
        .route("/clients/{email}", delete(delete_client_by_email))
        .with_state(container)
}

async fn create_client(
    State(container): State<Arc<ClientContainer>>,
    Json(payload): Json<CreateClient>,
) -> Result<StatusCode, AppError> {
    container.logic.create_client(payload).await?;
    Ok(StatusCode::CREATED)
}

async fn get_clients(
    State(container): State<Arc<ClientContainer>>,
    Json(filter): Json<FilterClient>,
) -> Json<Vec<Client>> {
    let result: Vec<Client> = container.logic.get_clients(filter).await;
    Json(result)
}

#[allow(dead_code)]
async fn get_client_by_email(
    State(container): State<Arc<ClientContainer>>,
    Path(email): Path<String>,
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
