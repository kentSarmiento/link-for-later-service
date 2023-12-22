use axum::{extract::State, http::StatusCode, response::IntoResponse, routing, Json, Router};

use crate::{
    state::AppState,
    types::{
        dto::{AuthResponse, UserInfoRequest},
        entity::UserInfoBuilder,
    },
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
    Json(payload): Json<UserInfoRequest>,
) -> impl IntoResponse {
    let users_repo = app_state.users_repo().clone();
    let user_info = UserInfoBuilder::new(payload.email(), payload.password()).build();
    match app_state
        .users_service()
        .register(Box::new(users_repo), &user_info)
        .await
    {
        Ok(info) => (StatusCode::CREATED, Json(info)).into_response(),
        Err(e) => {
            tracing::error!("Error: {}", e);
            e.into_response()
        }
    }
}

async fn login(
    State(app_state): State<AppState>,
    Json(payload): Json<UserInfoRequest>,
) -> impl IntoResponse {
    let users_repo = app_state.users_repo().clone();
    let user_info = UserInfoBuilder::new(payload.email(), payload.password()).build();
    match app_state
        .users_service()
        .login(Box::new(users_repo), &user_info)
        .await
    {
        Ok(token) => {
            let response = AuthResponse::new(token.jwt());
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => {
            tracing::error!("Error: {}", e);
            e.into_response()
        }
    }
}
