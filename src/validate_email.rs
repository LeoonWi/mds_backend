use std::sync::LazyLock;

use regex::Regex;

use crate::models::error::AppError;

static EMAIL_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?i)[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}$").unwrap());

pub fn validate_email(email: String) -> Result<String, AppError> {
    let email = email.trim();
    if email.is_empty() || email.len() > 254 {
        // По RFC 5321 максимальная длина email 254 символа
        return Err(AppError::BadRequest("Incorrect length Email".to_owned()));
    }

    if !EMAIL_REGEX.is_match(email) {
        return Err(AppError::BadRequest("Incorrect format Email".to_owned()));
    }

    Ok(email.to_owned())
}
