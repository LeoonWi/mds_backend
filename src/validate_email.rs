use std::sync::LazyLock;

use regex::Regex;

use crate::models::error::AppError;

static EMAIL_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?i)[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}$").unwrap());

pub fn validate_email(email: String) -> Result<String, AppError> {
    let email = email.replace(" ", "");
    if email.is_empty() || email.len() > 254 {
        // По RFC 5321 максимальная длина email 254 символа
        return Err(AppError::BadRequest("Incorrect length Email".to_owned()));
    }

    if !EMAIL_REGEX.is_match(&email) {
        return Err(AppError::BadRequest("Incorrect format Email".to_owned()));
    }

    Ok(email)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_correct_email() -> Result<(), AppError> {
        // Given
        let email = "webcastle@yandex.ru".to_owned();
        let email_with_space = "web castl e@yan dex.r u".to_owned();

        // When
        let correct_email = validate_email(email.clone()).inspect_err(|e| eprintln!("{:?}", e))?;
        let correct_email_with_space =
            validate_email(email_with_space.clone()).inspect_err(|e| eprintln!("{:?}", e))?;

        // Then
        println!("input: {email}\toutput: {correct_email}");
        println!("input: {email_with_space}\toutput: {correct_email_with_space}");

        assert_eq!(email, correct_email);
        assert_eq!(email, correct_email_with_space);

        Ok(())
    }

    #[test]
    fn input_uncorrect_email() -> Result<(), AppError> {
        // Given
        let email = "@yandex.ru".to_owned();

        // When
        let correct_email = validate_email(email.clone()).inspect_err(|e| eprintln!("{:?}", e));

        // Then
        assert_ne!(email, format! {"{:?}", correct_email.unwrap_err()});

        Ok(())
    }
}
