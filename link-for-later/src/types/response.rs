use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Login {
    pub token: String,
}

impl IntoResponse for super::AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Self::NotSupported => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            Self::ServerError => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            Self::DatabaseError => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            Self::ItemNotFound => (StatusCode::NOT_FOUND, self.to_string()),
            Self::UserAlreadyExists => (StatusCode::BAD_REQUEST, self.to_string()),
            Self::UserNotFound => (StatusCode::BAD_REQUEST, self.to_string()),
            Self::InvalidPassword => (StatusCode::UNAUTHORIZED, self.to_string()),
            Self::AuthorizationError => (StatusCode::UNAUTHORIZED, self.to_string()),
            #[cfg(test)]
            Self::TestError => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
