use std::sync::Arc;

use axum::extract::Path;
use axum::{Extension, middleware};
use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{delete, get, post},
};
use serde::Deserialize;

use crate::di::service_container::ServiceContainer;
use crate::httpserver::guard::guard;
use crate::models::employee::Role;
use crate::models::request_storage::RequestStorage;
use crate::models::{error::AppError, service::Service};

pub fn service_router(container: Arc<ServiceContainer>) -> Router {
    Router::new()
        .route("/services", post(create_service))
        .route("/services/{name}", delete(delete_service))
        .layer(middleware::from_fn(guard))
        .route("/services", get(get_services))
        .route("/services/{name}", get(get_service))
        .with_state(container)
}

#[derive(Deserialize)]
struct CreateService {
    name: String,
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

// PROTECTED

async fn create_service(
    Extension(request_storage): Extension<RequestStorage>,
    State(container): State<Arc<ServiceContainer>>,
    Json(payload): Json<CreateService>,
) -> Result<StatusCode, AppError> {
    if request_storage.role < Role::Manager {
        return Err(AppError::Forbidden);
    }

    container.service.create(payload.name).await?;

    Ok(StatusCode::CREATED)
}

async fn delete_service(
    Extension(request_storage): Extension<RequestStorage>,
    State(container): State<Arc<ServiceContainer>>,
    Path(name): Path<String>,
) -> Result<StatusCode, AppError> {
    if request_storage.role < Role::Manager {
        return Err(AppError::Forbidden);
    }
    container.service.delete_service(name).await?;

    Ok(StatusCode::OK)
}
