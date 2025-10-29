use std::sync::Arc;

use axum::extract::Path;
use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{delete, get, post},
};
use serde::Deserialize;

use crate::di::service_container::ServiceContainer;
use crate::models::{error::AppError, service::Service};

pub fn service_router(container: Arc<ServiceContainer>) -> Router {
    Router::new()
        .route("/services", post(create_service))
        .route("/services", get(get_services))
        .route("/services/{name}", get(get_service))
        .route("/services/{name}", delete(delete_service))
        .with_state(container)
}

#[derive(Deserialize)]
struct CreateService {
    name: String,
}

async fn create_service(
    State(container): State<Arc<ServiceContainer>>,
    Json(payload): Json<CreateService>,
) -> Result<StatusCode, AppError> {
    container.service.create(payload.name).await?;

    Ok(StatusCode::CREATED)
}

async fn get_services(
    State(container): State<Arc<ServiceContainer>>,
) -> Result<Json<Vec<Service>>, AppError> {
    container.service.get().await.map(Json)
}

async fn get_service(
    State(container): State<Arc<ServiceContainer>>,
    Path(name): Path<String>,
) -> Result<Json<Service>, AppError> {
    container.service.get_by_name(name).await.map(Json)
}

async fn delete_service(
    State(container): State<Arc<ServiceContainer>>,
    Path(name): Path<String>,
) -> Result<StatusCode, AppError> {
    container.service.delete_service(name).await?;

    Ok(StatusCode::OK)
}
