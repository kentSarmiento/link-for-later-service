use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use crate::types::AppError;

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Self::ServerError | Self::DatabaseError => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            Self::LinkNotFound => (StatusCode::NOT_FOUND, self.to_string()),
            Self::UserAlreadyExists
            | Self::UserNotFound
            | Self::InvalidEmail
            | Self::InvalidUrl => (StatusCode::BAD_REQUEST, self.to_string()),
            Self::AuthorizationError | Self::IncorrectPassword => {
                (StatusCode::UNAUTHORIZED, self.to_string())
            }
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
