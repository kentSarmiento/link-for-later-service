use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use crate::types::AppError;

#[allow(clippy::cognitive_complexity)]
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let error_message = self.to_string();
        tracing::info!("{}", error_message);
        let (status, error_message) = match self {
            Self::LinkNotFound(ref e) => {
                tracing::debug!("{}: {}", error_message, e.to_string());
                (StatusCode::NOT_FOUND, error_message)
            }
            Self::UserAlreadyExists(ref e) => {
                tracing::debug!("{}: {}", error_message, e.to_string());
                (StatusCode::BAD_REQUEST, error_message)
            }
            Self::UserNotFound(ref e) => {
                tracing::debug!("{}: {}", error_message, e.to_string());
                (StatusCode::BAD_REQUEST, error_message)
            }
            Self::IncorrectPassword(ref e) => {
                tracing::debug!("{}: {}", error_message, e.to_string());
                (StatusCode::UNAUTHORIZED, error_message)
            }
            Self::Authorization(ref e) => {
                tracing::debug!("{}: {}", error_message, e.to_string());
                (StatusCode::UNAUTHORIZED, error_message)
            }
            Self::Validation(ref e) => {
                tracing::debug!("{}: {}", error_message, e.to_string());
                (StatusCode::BAD_REQUEST, error_message)
            }
            Self::Database(ref e) => {
                tracing::debug!("{}: {}", error_message, e.to_string());
                (StatusCode::INTERNAL_SERVER_ERROR, error_message)
            }
            Self::Server(ref e) => {
                tracing::debug!("{}: {}", error_message, e.to_string());
                (StatusCode::INTERNAL_SERVER_ERROR, error_message)
            }

            #[cfg(test)]
            Self::Test => (StatusCode::INTERNAL_SERVER_ERROR, error_message),
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
            AppError::Server("a server operation failed".into())
                .into_response()
                .status(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            AppError::Database("a database operation failed".into())
                .into_response()
                .status(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            AppError::LinkNotFound("link".into())
                .into_response()
                .status(),
            StatusCode::NOT_FOUND
        );
        assert_eq!(
            AppError::UserAlreadyExists("user".into())
                .into_response()
                .status(),
            StatusCode::BAD_REQUEST
        );
        assert_eq!(
            AppError::UserNotFound("user".into())
                .into_response()
                .status(),
            StatusCode::BAD_REQUEST
        );
        assert_eq!(
            AppError::IncorrectPassword("user".into())
                .into_response()
                .status(),
            StatusCode::UNAUTHORIZED
        );
        assert_eq!(
            AppError::Authorization("an authorization error occurred".into())
                .into_response()
                .status(),
            StatusCode::UNAUTHORIZED
        );
        assert_eq!(
            AppError::Validation("a validation error occurred".into())
                .into_response()
                .status(),
            StatusCode::BAD_REQUEST
        );
    }
}
