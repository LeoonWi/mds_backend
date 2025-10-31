use std::sync::Arc;

use axum::extract::{Path, State};
use axum::routing::{get, patch, post};
use axum::{Extension, Json, middleware};
use axum::{Router, http::StatusCode};

use crate::di::employee_container::EmployeeContainer;
use crate::httpserver::guard::guard;
use crate::jwt::{self, JWTResponse};
use crate::models::employee::{
    CreateEmployee, Employee, EmployeeWithService, FilterEmployee, LoginEmployee, Role,
};
use crate::models::error::AppError;
use crate::models::request_storage::RequestStorage;

pub fn employee_router(container: Arc<EmployeeContainer>) -> Router {
    Router::new()
        .route("/employee", get(get_employees))
        .route("/employee_with_services", get(get_employees_with_services))
        .route("/employee/dismiss/{email}", patch(dismiss_employee))
        .route("/employee/{email}/set_role={role}", patch(change_role))
        .route("/employee/{id}/add_service={service}", patch(add_service))
        .route(
            "/employee/{id}/remove_service={service}",
            patch(remove_service),
        )
        .route("/employee/refresh_token", post(refresh_jwt_employee))
        .route("/employee", post(create_employee))
        .layer(middleware::from_fn(guard))
        .route("/employee/login", post(login_employee))
        .with_state(container)
}

async fn login_employee(
    State(container): State<Arc<EmployeeContainer>>,
    Json(payload): Json<LoginEmployee>,
) -> Result<Json<JWTResponse>, AppError> {
    let (user_id, role) = container.logic.login(payload).await?;
    let token = jwt::generate(user_id, Some(role))?;
    Ok(Json(token))
}

// PROTECTED

async fn create_employee(
    Extension(request_storage): Extension<RequestStorage>,
    State(container): State<Arc<EmployeeContainer>>,
    Json(payload): Json<CreateEmployee>,
) -> Result<StatusCode, AppError> {
    if request_storage.role < Role::Manager {
        return Err(AppError::Forbidden);
    }

    container.logic.create_employee(payload).await?;

    Ok(StatusCode::CREATED)
}

async fn refresh_jwt_employee(
    Extension(request_storage): Extension<RequestStorage>,
) -> Result<Json<JWTResponse>, AppError> {
    let token = jwt::generate(request_storage.user_id, Some(request_storage.role))?;
    Ok(Json(token))
}

async fn get_employees(
    Extension(request_storage): Extension<RequestStorage>,
    State(container): State<Arc<EmployeeContainer>>,
    Json(filter): Json<FilterEmployee>,
) -> Result<Json<Vec<Employee>>, AppError> {
    if request_storage.role < Role::Manager {
        return Err(AppError::Forbidden);
    }
    Ok(Json(container.logic.get_employees(filter).await))
}

async fn get_employees_with_services(
    Extension(request_storage): Extension<RequestStorage>,
    State(container): State<Arc<EmployeeContainer>>,
    Json(filter): Json<FilterEmployee>,
) -> Result<Json<Vec<EmployeeWithService>>, AppError> {
    if request_storage.role < Role::Manager {
        return Err(AppError::Forbidden);
    }
    Ok(Json(container.logic.get_with_services(filter).await))
}

#[allow(dead_code)]
async fn get_employee_by_email(
    Extension(request_storage): Extension<RequestStorage>,
    State(container): State<Arc<EmployeeContainer>>,
    Path(email): Path<String>,
) -> Result<Json<Employee>, AppError> {
    if request_storage.role < Role::Manager {
        return Err(AppError::Forbidden);
    }
    container.logic.get_employee_by_email(email).await.map(Json)
}

async fn dismiss_employee(
    Extension(request_storage): Extension<RequestStorage>,
    State(container): State<Arc<EmployeeContainer>>,
    Path(email): Path<String>,
) -> Result<StatusCode, AppError> {
    if request_storage.role < Role::Manager {
        return Err(AppError::Forbidden);
    }

    container
        .logic
        .dismiss_employee(request_storage.role, email)
        .await?;
    Ok(StatusCode::OK)
}

async fn change_role(
    Extension(request_storage): Extension<RequestStorage>,
    State(container): State<Arc<EmployeeContainer>>,
    Path((email, role)): Path<(String, Role)>,
) -> Result<StatusCode, AppError> {
    if request_storage.role < Role::Manager {
        return Err(AppError::Forbidden);
    }

    container
        .logic
        .change_role(request_storage.role, email, role)
        .await?;

    Ok(StatusCode::OK)
}

async fn add_service(
    Extension(request_storage): Extension<RequestStorage>,
    State(container): State<Arc<EmployeeContainer>>,
    Path((id, service)): Path<(i64, String)>,
) -> Result<StatusCode, AppError> {
    if request_storage.role < Role::Manager {
        return Err(AppError::Forbidden);
    }

    container.logic.add_service(id, service).await?;

    Ok(StatusCode::OK)
}

async fn remove_service(
    Extension(request_storage): Extension<RequestStorage>,
    State(container): State<Arc<EmployeeContainer>>,
    Path((id, service)): Path<(i64, String)>,
) -> Result<StatusCode, AppError> {
    if request_storage.role < Role::Manager {
        return Err(AppError::Forbidden);
    }

    container.logic.remove_service(id, service).await?;

    Ok(StatusCode::OK)
}
