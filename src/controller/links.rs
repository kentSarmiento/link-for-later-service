use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing, Router,
};
use serde_json::json;

use crate::types::state::Router as RouterState;

const LINKS_ROUTE: &str = "/v1/links";
const LINKS_ID_ROUTE: &str = "/v1/links/:id";

pub fn router() -> Router<RouterState> {
    Router::new()
        .route(LINKS_ROUTE, routing::get(list))
        .route(LINKS_ROUTE, routing::post(post))
        .route(LINKS_ID_ROUTE, routing::get(get))
        .route(LINKS_ID_ROUTE, routing::put(put))
        .route(LINKS_ID_ROUTE, routing::delete(delete))
}

async fn list(State(app_state): State<RouterState>) -> impl IntoResponse {
    let links_service = app_state.get_links_service();
    match links_service.list(&app_state).await {
        Ok(list) => Json(list).into_response(),
        Err(e) => {
            tracing::error!("Error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "msg": e.to_string() })),
            )
                .into_response()
        }
    }
}

async fn post(State(app_state): State<RouterState>) -> impl IntoResponse {
    let links_repo = app_state.get_links_repo();
    match links_repo.post().await {
        Ok(link) => Json(link).into_response(),
        Err(e) => {
            tracing::error!("Error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "msg": e.to_string() })),
            )
                .into_response()
        }
    }
}

async fn get(Path(id): Path<String>, State(app_state): State<RouterState>) -> impl IntoResponse {
    let links_repo = app_state.get_links_repo();
    match links_repo.get(&id).await {
        Ok(link) => Json(link).into_response(),
        Err(e) => {
            tracing::error!("Error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "msg": e.to_string() })),
            )
                .into_response()
        }
    }
}

async fn put(Path(id): Path<String>, State(app_state): State<RouterState>) -> impl IntoResponse {
    let links_repo = app_state.get_links_repo();
    match links_repo.put(&id).await {
        Ok(link) => Json(link).into_response(),
        Err(e) => {
            tracing::error!("Error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "msg": e.to_string() })),
            )
                .into_response()
        }
    }
}

async fn delete(Path(id): Path<String>, State(app_state): State<RouterState>) -> impl IntoResponse {
    let links_repo = app_state.get_links_repo();
    match links_repo.delete(&id).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => {
            tracing::error!("Error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "msg": e.to_string() })),
            )
                .into_response()
        }
    }
}
