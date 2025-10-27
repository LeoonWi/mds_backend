use phonenumber::country;

use crate::models::error::AppError;

pub fn phoneparse<S: AsRef<str>>(
    country: Option<country::Id>,
    string: S,
) -> Result<String, AppError> {
    let phone_number = phonenumber::parse(country, string).map_err(|_| {
        tracing::error!("Failed to parse phone number");
        AppError::BadRequest("Failed to parse phone number".to_owned())
    })?;

    let phone_number = phone_number.to_string();

    if phone_number.len() < 12 {
        tracing::error!("Incorrect length phone number");
        return Err(AppError::BadRequest(
            "Incorrect length phone number".to_owned(),
        ));
    }

    Ok(phone_number)
}

#[cfg(test)]
mod tests {
    use phonenumber::country::Id::RU;

    use crate::phoneparse::phoneparse;

    #[test]
    fn correct_phone_with_lead_8() {
        // Given
        let phone_with_8 = "89787548353".to_owned();

        // When
        let parse_phone = phoneparse(Some(RU), &phone_with_8).unwrap();

        // Then
        println!("input: {phone_with_8}\touput: {parse_phone}");
        assert_eq!(parse_phone, "+79787548353".to_owned());
    }

    #[test]
    fn correct_phone_with_lead_7() {
        // Given
        let phone_with_7 = "79787548353".to_owned();

        // When
        let parse_phone = phoneparse(Some(RU), &phone_with_7).unwrap();

        // Then
        println!("input: {phone_with_7}\touput: {parse_phone}");
        assert_eq!(parse_phone, "+79787548353".to_owned());
    }

    #[test]
    fn correct_phone_without_lead() {
        // Given
        let phone = "9787548353".to_owned();

        // When
        let parse_phone = phoneparse(Some(RU), &phone).unwrap();

        // Then
        println!("input: {phone}\touput: {parse_phone}");
        assert_eq!(parse_phone, "+79787548353".to_owned());
    }

    #[test]
    fn correct_phone_with_whitespace() {
        // Given
        let phone_with_space = "+  79 787   54 8  3 5   3".to_owned();

        // When
        let parse_phone = phoneparse(Some(RU), &phone_with_space).unwrap();

        // Then
        println!("input: {phone_with_space}\touput: {parse_phone}");
        assert_eq!(parse_phone, "+79787548353".to_owned());
    }

    #[test]
    fn correct_phone_with_bracket() {
        // Given
        let phone_with_bracket = "+7(978)754-83-53".to_owned();

        // When
        let parse_phone = phoneparse(Some(RU), &phone_with_bracket).unwrap();

        // Then
        println!("input: {phone_with_bracket}\touput: {parse_phone}");
        assert_eq!(parse_phone, "+79787548353".to_owned());
    }

    #[test]
    fn uncorrect_phone() {
        // Given
        let phone_without_tail = "7978754835".to_owned();

        // When
        let parse_phone_without_tail = phoneparse(Some(RU), &phone_without_tail);

        // Then
        println!(
            "input: {:?}\toutput: {:?}",
            phone_without_tail, parse_phone_without_tail
        );
        assert_ne!(
            format!("{:?}", parse_phone_without_tail),
            "+79787548353".to_owned()
        );
    }
}
