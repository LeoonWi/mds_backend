use std::sync::Arc;

use axum::Json;
use axum::extract::{Path, State};
use axum::routing::{get, patch, post};
use axum::{Router, http::StatusCode};
use serde::Deserialize;

use crate::di::employee_container::EmployeeContainer;
use crate::models::employee::Employee;
use crate::models::error::AppError;

pub fn employee_router(container: Arc<EmployeeContainer>) -> Router {
    Router::new()
        .route("/employee", post(create_employee))
        .route("/employee", get(get_employees))
        .route("/employee/{email}", get(get_employee_by_email))
        .route("/employee/dismiss", patch(dismiss_employee))
        .with_state(container)
}

#[derive(Deserialize)]
struct CreateEmployee {
    pub name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub email: String,
    pub password: String,
}

async fn create_employee(
    State(container): State<Arc<EmployeeContainer>>,
    Json(payload): Json<CreateEmployee>,
) -> Result<StatusCode, AppError> {
    container
        .logic
        .create_employee(
            payload.name,
            payload.last_name,
            payload.middle_name,
            payload.email,
            payload.password,
        )
        .await?;

    Ok(StatusCode::OK)
}

async fn get_employees(State(container): State<Arc<EmployeeContainer>>) -> Json<Vec<Employee>> {
    Json(container.logic.get_employees().await)
}

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
