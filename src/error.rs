use actix_http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use std::fmt::{Display, Formatter};

/// The application error type.
#[derive(Debug)]
pub enum AppError {
    /// The [serde_json::Error] variant.
    SerdeError(serde_json::Error),
    /// The [reqwest::Error] variant.
    ReqwestError(reqwest::Error),
    /// Internal server error. The string value of this variant is not sent to the client.
    InternalError(String),
    /// The error raised due to bad data sent by the client. The first element of the tuple is the
    /// HTTP [StatusCode] (defaults to [StatusCode::BAD_REQUEST]) and the second element is the
    /// string message that is sent to the client.
    ///
    /// # Caution
    ///
    /// The string value of this variant is sent to the client. It must not contain any sensitive
    /// details.
    ClientError(StatusCode, String),
}

impl From<serde_json::Error> for AppError {
    fn from(error: serde_json::Error) -> Self {
        AppError::SerdeError(error)
    }
}

impl From<reqwest::Error> for AppError {
    fn from(error: reqwest::Error) -> Self {
        AppError::ReqwestError(error)
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for AppError {
    /// Returns the HTTP [StatusCode] for the error.
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::InternalError(_) | AppError::SerdeError(_) | AppError::ReqwestError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            AppError::ClientError(status_code, _) => *status_code,
        }
    }

    /// Returns the [HttpResponse] for the error.
    fn error_response(&self) -> HttpResponse {
        let mut response_builder = HttpResponse::build(self.status_code());

        match self {
            AppError::InternalError(_) | AppError::SerdeError(_) | AppError::ReqwestError(_) => {
                response_builder.body("Internal server error")
            }
            AppError::ClientError(_, message) => response_builder.body(message.to_string()),
        }
    }
}

// Allows creating simple client errors from a string slice.
impl From<&str> for AppError {
    fn from(value: &str) -> Self {
        AppError::ClientError(StatusCode::BAD_REQUEST, value.to_string())
    }
}

// Allows creating simple client errors from a string value.
impl From<String> for AppError {
    fn from(value: String) -> Self {
        AppError::ClientError(StatusCode::BAD_REQUEST, value)
    }
}
