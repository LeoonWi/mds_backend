use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::{jwt::verify, models::request_storage::RequestStorage};

pub async fn guard(mut request: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    let token = match request.headers().get(axum::http::header::AUTHORIZATION) {
        Some(value) => value.to_str().ok(),
        None => None,
    };

    let token = match token {
        Some(header) if header.starts_with("Bearer ") => &header["Bearer ".len()..],
        _ => return Err(StatusCode::UNAUTHORIZED),
    };

    let claims = verify(token)?;

    let user_id = claims
        .sub
        .parse::<i64>()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let role = claims.role;

    let request_storage = RequestStorage { user_id, role };

    request.extensions_mut().insert(request_storage);

    Ok(next.run(request).await)
}
