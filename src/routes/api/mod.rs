pub mod user;

#[derive(Debug, Clone, serde::Serialize, PartialEq)]
pub struct ApiErrorMessage<'a> {
    message: &'a str,
    code: u32,
    field: &'a str,
}

impl<'a> ApiErrorMessage<'a> {
    pub fn user_already_exists() -> Self {
        Self {
            message: "user already exists",
            code: 1,
            field: "username",
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, PartialEq)]
pub struct ApiResponse<'a> {
    success: bool,
    message: &'a str,
    errors: Option<Vec<ApiErrorMessage<'a>>>,
}

impl<'a> ApiResponse<'a> {
    pub fn new(success: bool, message: &'a str) -> Self {
        Self {
            success,
            message,
            errors: None,
        }
    }

    pub fn add_error(&mut self, error: ApiErrorMessage<'a>) {
        match self.errors.take() {
            None => self.errors = Some(vec![error]),
            Some(mut v) => v.push(error),
        }
    }

    pub fn with_errors(success: bool, message: &'a str, errors: Vec<ApiErrorMessage<'a>>) -> Self {
        Self {
            success,
            message,
            errors: Some(errors),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_error() {
        let mut response = ApiResponse::new(false, "test error");
        response.add_error(ApiErrorMessage::user_already_exists());

        assert_eq!(
            response,
            ApiResponse {
                success: false,
                message: "test error",
                errors: Some(vec![ApiErrorMessage::user_already_exists()])
            }
        )
    }
}
