use crate::error::ApiError;

pub fn validate_post(title: &str, content: &str) -> Result<(), ApiError> {
    if title.trim().is_empty() {
        return Err(ApiError::InvalidInput(String::from(
            "title cannot be empty",
        )));
    }

    if content.trim().is_empty() {
        return Err(ApiError::InvalidInput(String::from(
            "content cannot be empty",
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::validate_post;

    #[test]
    fn accepts_valid_post() {
        assert!(validate_post("Hello", "Some content").is_ok());
    }

    #[test]
    fn rejects_blank_title() {
        assert!(validate_post("   ", "Some content").is_err());
    }

    #[test]
    fn rejects_blank_content() {
        assert!(validate_post("Hello", " \n\t ").is_err());
    }
}
