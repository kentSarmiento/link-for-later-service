use axum::{extract::State, http::StatusCode, response::IntoResponse, routing, Json, Router};

use crate::{
    state::AppState,
    types::{AppError, LoginRequest, RegisterRequest},
};

const USERS_SIGNUP_ROUTE: &str = "/v1/users/register";
const USERS_LOGIN_ROUTE: &str = "/v1/users/login";

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route(USERS_SIGNUP_ROUTE, routing::post(register))
        .route(USERS_LOGIN_ROUTE, routing::post(login))
        .with_state(state)
}

async fn register(
    State(app_state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> impl IntoResponse {
    match app_state.users_repo().add(&payload.into()).await {
        Ok(link) => (StatusCode::CREATED, Json(link)).into_response(),
        Err(e) => {
            tracing::error!("Error: {}", e);
            e.into_response()
        }
    }
}

async fn login(
    State(_app_state): State<AppState>,
    Json(_payload): Json<LoginRequest>,
) -> impl IntoResponse {
    AppError::NotSupported.into_response()
}
