use std::sync::Arc;

use axum::Json;
use axum::extract::{Path, State};
use axum::routing::{get, patch, post};
use axum::{Router, http::StatusCode};

use crate::di::employee_container::EmployeeContainer;
use crate::models::employee::{CreateEmployee, Employee, FilterEmployee};
use crate::models::error::AppError;

pub fn employee_router(container: Arc<EmployeeContainer>) -> Router {
    Router::new()
        .route("/employee", post(create_employee))
        .route("/employee", get(get_employees))
        .route("/employee/dismiss/{email}", patch(dismiss_employee))
        .with_state(container)
}

async fn create_employee(
    State(container): State<Arc<EmployeeContainer>>,
    Json(payload): Json<CreateEmployee>,
) -> Result<StatusCode, AppError> {
    container.logic.create_employee(payload).await?;

    Ok(StatusCode::CREATED)
}

async fn get_employees(
    State(container): State<Arc<EmployeeContainer>>,
    Json(filter): Json<FilterEmployee>,
) -> Json<Vec<Employee>> {
    Json(container.logic.get_employees(filter).await)
}

#[allow(dead_code)]
async fn get_employee_by_email(
    State(container): State<Arc<EmployeeContainer>>,
    Path(email): Path<String>,
) -> Result<Json<Employee>, AppError> {
    container.logic.get_employee_by_email(email).await.map(Json)
}

async fn dismiss_employee(
    State(container): State<Arc<EmployeeContainer>>,
    Path(email): Path<String>,
) -> Result<StatusCode, AppError> {
    container.logic.dismiss_employee(email).await?;

    Ok(StatusCode::OK)
}
