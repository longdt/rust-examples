use ntex::http::StatusCode;
use std::fmt::{Display, Formatter};

use ntex::web::{HttpRequest, HttpResponse, WebResponseError};
use serde::Serialize;
use sqlx::Error;

#[derive(Debug, Serialize)]
pub enum EzyTutorError {
    DBError(String),
    ActixError(String),
    NotFound(String),
}

impl From<sqlx::Error> for EzyTutorError {
    fn from(value: Error) -> Self {
        EzyTutorError::DBError(value.to_string())
    }
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    error_message: String,
}

impl Display for EzyTutorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl WebResponseError for EzyTutorError {
    fn status_code(&self) -> StatusCode {
        match self {
            EzyTutorError::DBError(_) | EzyTutorError::ActixError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            EzyTutorError::NotFound(_) => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self, _: &HttpRequest) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(&ErrorResponse {
            error_message: self.to_string(),
        })
    }
}
