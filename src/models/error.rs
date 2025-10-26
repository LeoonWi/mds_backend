// use axum::http::StatusCode;

// pub type ErrorResponse = (StatusCode, String);

#[derive(Debug)]
pub enum AppError {
    Conflict,
    BadRequest(String),
    NotFound,
    InternalServerError,
}
