use crate::error::ApiError;

pub fn validate_comment(content: &str) -> Result<(), ApiError> {
    let length = content.trim().chars().count();

    if length == 0 {
        return Err(ApiError::InvalidInput(String::from(
            "comment cannot be empty",
        )));
    }

    if length > 2000 {
        return Err(ApiError::InvalidInput(String::from(
            "comment cannot exceed 2000 characters",
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::validate_comment;

    #[test]
    fn accepts_a_valid_comment() {
        assert!(validate_comment("Nice post!").is_ok());
    }

    #[test]
    fn rejects_a_blank_comment() {
        assert!(validate_comment("  \n ").is_err());
    }

    #[test]
    fn rejects_a_comment_over_2000_characters() {
        let comment = "a".repeat(2001);

        assert!(validate_comment(&comment).is_err());
    }
}
