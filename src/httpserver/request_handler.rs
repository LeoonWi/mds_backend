use std::sync::Arc;

use axum::Json;
use axum::Router;
use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::delete;
use axum::routing::patch;
use axum::routing::{get, post};

use crate::di::request_container::RequestContainer;
use crate::models::error::AppError;
use crate::models::request::{CreateRequest, FilterRequest, Priority, Request, Status};

pub fn request_router(container: Arc<RequestContainer>) -> Router {
    Router::new()
        .route("/requests", post(create_request))
        .route("/requests", get(get_requests))
        .route("/requests/{id}/set_status={status}", patch(set_status))
        .route(
            "/requests/{id}/set_priority={priority}",
            patch(set_priority),
        )
        .route(
            "/requests/{id}/set_employee={employee_id}",
            patch(set_employee),
        )
        .route("/requests/{id}", delete(delete_request))
        .with_state(container)
}

async fn create_request(
    State(container): State<Arc<RequestContainer>>,
    Json(payload): Json<CreateRequest>,
) -> Result<StatusCode, AppError> {
    container
        .request
        .create(payload)
        .await
        .map(|_| StatusCode::CREATED)
}

async fn get_requests(
    State(container): State<Arc<RequestContainer>>,
    Json(filter): Json<FilterRequest>,
) -> Json<Vec<Request>> {
    let result = container.request.get(filter).await;
    Json(result)
}

async fn set_status(
    State(container): State<Arc<RequestContainer>>,
    Path((id, status)): Path<(i64, Status)>,
) -> Result<StatusCode, AppError> {
    container
        .request
        .set_status(id, status)
        .await
        .map(|_| StatusCode::OK)
}

async fn set_priority(
    State(container): State<Arc<RequestContainer>>,
    Path((id, priority)): Path<(i64, Priority)>,
) -> Result<StatusCode, AppError> {
    container
        .request
        .set_priority(id, priority)
        .await
        .map(|_| StatusCode::OK)
}

async fn set_employee(
    State(container): State<Arc<RequestContainer>>,
    Path((id, employee_id)): Path<(i64, i64)>,
) -> Result<StatusCode, AppError> {
    container
        .request
        .change_employee(id, employee_id)
        .await
        .map(|_| StatusCode::OK)
}

async fn delete_request(
    State(container): State<Arc<RequestContainer>>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    container.request.delete(id).await.map(|_| StatusCode::OK)
}
