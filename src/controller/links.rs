use axum::{
    extract::{self, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing, Json, Router,
};
use serde_json::json;

use crate::types::{request::PostLink as PostLinkRequest, state::Router as RouterState};

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
                Json(json!({ "msg": "An error occurred." })),
            )
                .into_response()
        }
    }
}

async fn post(
    State(app_state): State<RouterState>,
    Json(payload): extract::Json<PostLinkRequest>,
) -> impl IntoResponse {
    let links_service = app_state.get_links_service();

    match links_service
        .post(&app_state, &payload.to_string().into())
        .await
    {
        Ok(link) => (StatusCode::CREATED, Json(link)).into_response(),
        Err(e) => {
            tracing::error!("Error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "msg": "An error occurred." })),
            )
                .into_response()
        }
    }
}

async fn get(Path(id): Path<String>, State(app_state): State<RouterState>) -> impl IntoResponse {
    let links_service = app_state.get_links_service();
    match links_service.get(&id, &app_state).await {
        Ok(link) => Json(link).into_response(),
        Err(e) => {
            tracing::error!("Error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "msg": "An error occurred." })),
            )
                .into_response()
        }
    }
}

async fn put(Path(id): Path<String>, State(app_state): State<RouterState>) -> impl IntoResponse {
    let links_service = app_state.get_links_service();
    match links_service.put(&id, &app_state).await {
        Ok(link) => Json(link).into_response(),
        Err(e) => {
            tracing::error!("Error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "msg": "An error occurred." })),
            )
                .into_response()
        }
    }
}

async fn delete(Path(id): Path<String>, State(app_state): State<RouterState>) -> impl IntoResponse {
    let links_service = app_state.get_links_service();
    match links_service.delete(&id, &app_state).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => {
            tracing::error!("Error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "msg": "An error occurred." })),
            )
                .into_response()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use http_body_util::BodyExt;

    use crate::types::{
        links::LinkItem, repository::MockLinks as MockRepository, service::MockLinks as MockService,
    };

    use super::*;

    #[tokio::test]
    async fn test_get_links_empty() {
        let mut mock_links_service = MockService::new();
        let mock_links_repo = MockRepository::new();
        mock_links_service
            .expect_list()
            .times(1)
            .returning(|_| Ok(vec![]));

        let app_state = RouterState::new(Arc::new(mock_links_service), Arc::new(mock_links_repo));

        let response = list(State(app_state)).await;

        let (parts, body) = response.into_response().into_parts();
        assert_eq!(StatusCode::OK, parts.status);

        let body = body.collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"[]");
    }

    #[tokio::test]
    async fn test_get_links_non_empty() {
        let mut mock_links_service = MockService::new();
        let mock_links_repo = MockRepository::new();
        let item: LinkItem = "http://link".into();

        mock_links_service
            .expect_list()
            .times(1)
            .returning(move |_| Ok(vec![item.clone()]));

        let app_state = RouterState::new(Arc::new(mock_links_service), Arc::new(mock_links_repo));

        let response = list(State(app_state)).await;

        let (parts, body) = response.into_response().into_parts();
        assert_eq!(StatusCode::OK, parts.status);

        let body = body.collect().await.unwrap().to_bytes();
        let body = std::str::from_utf8(&body).unwrap();
        assert!(body.contains("http://link"));
    }

    #[tokio::test]
    async fn test_get_links_service_error() {
        let mut mock_links_service = MockService::new();
        let mock_links_repo = MockRepository::new();
        mock_links_service
            .expect_list()
            .times(1)
            .returning(|_| Err("A service error occurred.".into()));

        let app_state = RouterState::new(Arc::new(mock_links_service), Arc::new(mock_links_repo));

        let response = list(State(app_state)).await;

        let (parts, body) = response.into_response().into_parts();
        assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, parts.status);

        let body = body.collect().await.unwrap().to_bytes();
        let body = std::str::from_utf8(&body).unwrap();
        assert!(body.contains("An error occurred."));
    }
}
