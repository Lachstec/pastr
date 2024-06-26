use actix_web::{error, HttpRequest, HttpResponse};

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
    #[serde(skip_serializing_if = "Option::is_none")]
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

pub fn json_deserialize_error_handler(
    err: error::JsonPayloadError,
    _req: &HttpRequest,
) -> error::Error {
    use error::JsonPayloadError;

    let response = match &err {
        JsonPayloadError::ContentType => HttpResponse::UnsupportedMediaType().json(
            ApiResponse::new(false, "invalid content-type. expected json"),
        ),
        JsonPayloadError::Deserialize(json_err) if json_err.is_data() => {
            HttpResponse::UnprocessableEntity()
                .json(ApiResponse::new(false, "malformed json payload"))
        }
        _ => HttpResponse::BadRequest().json(ApiResponse::new(false, "unexpected json body")),
    };
    error::InternalError::from_response(err, response).into()
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
