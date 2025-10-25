// use axum::http::StatusCode;

// pub type ErrorResponse = (StatusCode, String);

pub enum AppError {
    Conflict,
    BadRequest(String),
    NotFound,
    InternalServerError(String),
}
