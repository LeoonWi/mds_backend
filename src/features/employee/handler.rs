use std::sync::Arc;

use axum::{Json, extract::State};

use crate::{
    features::employee::logic::Logic,
    models::{
        employee::{CreateEmployee, Employee},
        error::ErrorResponse,
    },
};

pub async fn create_employee(
    State(logic): State<Arc<Logic>>,
    Json(payload): Json<CreateEmployee>,
) -> Result<Json<Employee>, ErrorResponse> {
    // TODO
    todo!()
}
