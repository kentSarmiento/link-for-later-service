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
            Self::ServerError => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            Self::DatabaseError(ref e) => {
                tracing::error!("Database error: {}", e.to_string());
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            Self::LinkNotFound => (StatusCode::NOT_FOUND, self.to_string()),
            Self::UserAlreadyExists
            | Self::UserNotFound
            | Self::InvalidEmail
            | Self::InvalidUrl => (StatusCode::BAD_REQUEST, self.to_string()),
            Self::IncorrectPassword => (StatusCode::UNAUTHORIZED, self.to_string()),
            Self::AuthorizationError(ref e) => {
                tracing::error!("Authorization error: {}", e.to_string());
                (StatusCode::UNAUTHORIZED, self.to_string())
            }
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_error_response() {
        assert_eq!(
            AppError::ServerError.into_response().status(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            AppError::DatabaseError("a database error occurred".into())
                .into_response()
                .status(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            AppError::ServerError.into_response().status(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            AppError::LinkNotFound.into_response().status(),
            StatusCode::NOT_FOUND
        );
        assert_eq!(
            AppError::UserAlreadyExists.into_response().status(),
            StatusCode::BAD_REQUEST
        );
        assert_eq!(
            AppError::UserNotFound.into_response().status(),
            StatusCode::BAD_REQUEST
        );
        assert_eq!(
            AppError::InvalidEmail.into_response().status(),
            StatusCode::BAD_REQUEST
        );
        assert_eq!(
            AppError::InvalidUrl.into_response().status(),
            StatusCode::BAD_REQUEST
        );
        assert_eq!(
            AppError::IncorrectPassword.into_response().status(),
            StatusCode::UNAUTHORIZED
        );
        assert_eq!(
            AppError::AuthorizationError("authorization error occurred".into())
                .into_response()
                .status(),
            StatusCode::UNAUTHORIZED
        );
    }
}
