use axum::http::StatusCode;

pub type ErrorResponse = (StatusCode, String);

#[derive(Debug)]
pub enum Error {
    Conflict(String),
    BadRequest(String),
    NotFound(String),
    InternalServerError(String),
}

impl Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::Conflict(_) => StatusCode::CONFLICT,
            Error::BadRequest(_) => StatusCode::BAD_REQUEST,
            Error::NotFound(_) => StatusCode::NOT_FOUND,
            Error::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn into_response(self) -> ErrorResponse {
        let status_code = self.status_code();
        let message = match self {
            Error::Conflict(msg) => msg,
            Error::BadRequest(msg) => msg,
            Error::NotFound(msg) => msg,
            Error::InternalServerError(msg) => msg,
        };
        (status_code, message)
    }
}
