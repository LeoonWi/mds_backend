use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};

use crate::models::role::Role;

struct Claims {
    sub: String,
    role: Option<Role>,
    exp: usize,
}

fn generate_token(user_id: i64, role: Option<Role>, secret: &str) -> (String, String) {
    let access_claims = Claims {
        sub: user_id.to_string(),
        role: role,
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

    (access_token, refresh_token)
}

fn verify_token(token: &str, secret: &str) -> Result<Claims, StatusCode> {
    let key = DecodingKey::from_secret(secret.as_bytes());
    let validation = Validation::new(jsonwebtoken::Algorithm::HS256);
    decode::<Claims>(token, &key, &validation)
        .map(|data| data.claims)
        .map_err(|_| StatusCode::UNAUTHORIZED)
}
