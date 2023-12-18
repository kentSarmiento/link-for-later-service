use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing, Router,
};
use serde_json::json;

use crate::repository::DynLinksRepo;

const LINKS_ROUTE: &str = "/v1/links";
const LINKS_ID_ROUTE: &str = "/v1/links/:id";

pub fn router() -> Router<DynLinksRepo> {
    Router::new()
        .route(LINKS_ROUTE, routing::get(list))
        .route(LINKS_ROUTE, routing::post(post))
        .route(LINKS_ID_ROUTE, routing::get(get))
        .route(LINKS_ID_ROUTE, routing::put(put))
        .route(LINKS_ID_ROUTE, routing::delete(delete))
}

async fn list(State(links_repo): State<DynLinksRepo>) -> impl IntoResponse {
    match links_repo.list().await {
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

async fn post(State(links_repo): State<DynLinksRepo>) -> impl IntoResponse {
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

async fn get(Path(id): Path<String>, State(links_repo): State<DynLinksRepo>) -> impl IntoResponse {
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

async fn put(Path(id): Path<String>, State(links_repo): State<DynLinksRepo>) -> impl IntoResponse {
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

async fn delete(
    Path(id): Path<String>,
    State(links_repo): State<DynLinksRepo>,
) -> impl IntoResponse {
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
