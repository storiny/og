use actix_http::StatusCode;
use actix_web::{
    HttpResponse,
    ResponseError,
};
use std::fmt::{
    Display,
    Formatter,
};
use tonic::Code;

/// The application error type.
#[derive(Debug)]
pub enum AppError {
    /// The [png::EncodingError] variant.
    PngEncodingError(png::EncodingError),
    /// The [sailfish::RenderError] variant.
    RenderTemplateError(sailfish::RenderError),
    /// The [tonic::Status] variant.
    TonicError(tonic::Status),
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

impl From<png::EncodingError> for AppError {
    fn from(error: png::EncodingError) -> Self {
        AppError::PngEncodingError(error)
    }
}

impl From<sailfish::RenderError> for AppError {
    fn from(error: sailfish::RenderError) -> Self {
        AppError::RenderTemplateError(error)
    }
}

impl From<tonic::Status> for AppError {
    fn from(error: tonic::Status) -> Self {
        AppError::TonicError(error)
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
            AppError::InternalError(_)
            | AppError::PngEncodingError(_)
            | AppError::RenderTemplateError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::TonicError(status) => match status.code() {
                Code::NotFound => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
            AppError::ClientError(status_code, _) => *status_code,
        }
    }

    /// Returns the [HttpResponse] for the error.
    fn error_response(&self) -> HttpResponse {
        let mut response_builder = HttpResponse::build(self.status_code());

        match self {
            AppError::InternalError(_)
            | AppError::PngEncodingError(_)
            | AppError::RenderTemplateError(_) => response_builder.body("Internal server error"),
            AppError::TonicError(status) => match status.code() {
                Code::NotFound => response_builder.body("Entity not found"),
                _ => response_builder.body("Internal gateway error"),
            },
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
