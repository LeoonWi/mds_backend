use std::env;

use axum::http::StatusCode;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::models::{employee::Role, error::AppError};

#[derive(Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: Role,
    pub exp: usize,
}

#[derive(Serialize)]
pub struct JWTResponse {
    pub access_token: String,
    pub refresh_token: String,
}

pub fn generate(user_id: i64, role: Option<Role>) -> Result<JWTResponse, AppError> {
    let secret = env::var("JWT_SECRET").map_err(|_| {
        tracing::error!("Failed load JWT_SECRET from .env");
        AppError::InternalServerError
    })?;

    let role = match role {
        None => Role::Guest,
        _ => role.unwrap(),
    };

    let access_claims = Claims {
        sub: user_id.to_string(),
        role: role.clone(),
        exp: (Utc::now() + Duration::minutes(15)).timestamp() as usize,
    };

    let refresh_claims = Claims {
        sub: user_id.to_string(),
        role: role,
        exp: (Utc::now() + Duration::days(7)).timestamp() as usize,
    };

    let key = EncodingKey::from_secret(secret.as_bytes());
    let access_token = encode(&Header::default(), &access_claims, &key).unwrap();
    let refresh_token = encode(&Header::default(), &refresh_claims, &key).unwrap();

    Ok(JWTResponse {
        access_token,
        refresh_token,
    })
}

pub fn verify(token: &str) -> Result<Claims, StatusCode> {
    let secret = env::var("JWT_SECRET").map_err(|_| {
        tracing::error!("Failed load JWT_SECRET from .env");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    let key = DecodingKey::from_secret(secret.as_bytes());
    let validation = Validation::new(jsonwebtoken::Algorithm::HS256);
    decode::<Claims>(token, &key, &validation)
        .map(|data| data.claims)
        .map_err(|err| match err.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        })
}
