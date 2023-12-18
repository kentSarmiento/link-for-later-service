use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use super::ServerError;

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Self::ItemNotFound => (StatusCode::NOT_FOUND, self.to_string()),
            Self::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}