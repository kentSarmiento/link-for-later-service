use axum::{
    extract::{self, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing, Json, Router,
};

use crate::types::{
    request::PostLink as PostLinkRequest, request::PutLink as PutLinkRequest, AppState,
};

const LINKS_ROUTE: &str = "/v1/links";
const LINKS_ID_ROUTE: &str = "/v1/links/:id";

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route(LINKS_ROUTE, routing::get(list))
        .route(LINKS_ROUTE, routing::post(post))
        .route(LINKS_ID_ROUTE, routing::get(get))
        .route(LINKS_ID_ROUTE, routing::put(put))
        .route(LINKS_ID_ROUTE, routing::delete(delete))
        .with_state(state)
}

async fn list(State(app_state): State<AppState>) -> impl IntoResponse {
    match app_state.get_links_service().list(&app_state).await {
        Ok(list) => Json(list).into_response(),
        Err(e) => {
            tracing::error!("Error: {}", e);
            e.into_response()
        }
    }
}

async fn post(
    State(app_state): State<AppState>,
    Json(payload): extract::Json<PostLinkRequest>,
) -> impl IntoResponse {
    match app_state
        .get_links_service()
        .post(&app_state, &payload.into())
        .await
    {
        Ok(link) => (StatusCode::CREATED, Json(link)).into_response(),
        Err(e) => {
            tracing::error!("Error: {}", e);
            e.into_response()
        }
    }
}

async fn get(State(app_state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    match app_state.get_links_service().get(&app_state, &id).await {
        Ok(link) => Json(link).into_response(),
        Err(e) => {
            tracing::error!("Error: {}", e);
            e.into_response()
        }
    }
}

async fn put(
    State(app_state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): extract::Json<PutLinkRequest>,
) -> impl IntoResponse {
    match app_state
        .get_links_service()
        .put(&app_state, &id, &payload.into())
        .await
    {
        Ok(link) => Json(link).into_response(),
        Err(e) => {
            tracing::error!("Error: {}", e);
            e.into_response()
        }
    }
}

async fn delete(State(app_state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    match app_state.get_links_service().delete(&app_state, &id).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => {
            tracing::error!("Error: {}", e);
            e.into_response()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use http_body_util::BodyExt;
    use serde_json::Value;

    use crate::types::{
        links::LinkItem, repository::MockLinks as MockRepository,
        service::MockLinks as MockService, AppError,
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

        let app_state = AppState::new(Arc::new(mock_links_service), Arc::new(mock_links_repo));

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
        let item = LinkItem::new("1", "http://link");

        mock_links_service
            .expect_list()
            .times(1)
            .returning(move |_| Ok(vec![item.clone()]));

        let app_state = AppState::new(Arc::new(mock_links_service), Arc::new(mock_links_repo));

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
            .returning(|_| Err(AppError::TestError));

        let app_state = AppState::new(Arc::new(mock_links_service), Arc::new(mock_links_repo));

        let response = list(State(app_state)).await;

        let (parts, body) = response.into_response().into_parts();
        assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, parts.status);

        let body = body.collect().await.unwrap().to_bytes();
        let body = std::str::from_utf8(&body).unwrap();
        assert!(body.contains("test error"));
    }

    #[tokio::test]
    async fn test_post_links() {
        let mut mock_links_service = MockService::new();
        let mock_links_repo = MockRepository::new();
        let request = PostLinkRequest::new("1", "http://link");
        let item: LinkItem = request.clone().into();

        mock_links_service
            .expect_post()
            .times(1)
            .returning(move |_, _| Ok(item.clone()));

        let app_state = AppState::new(Arc::new(mock_links_service), Arc::new(mock_links_repo));

        let response = post(State(app_state), Json(request)).await;

        let (parts, body) = response.into_response().into_parts();
        assert_eq!(StatusCode::CREATED, parts.status);

        let body = body.collect().await.unwrap().to_bytes();
        let body = std::str::from_utf8(&body).unwrap();
        let body: Value = serde_json::from_str(body).unwrap();

        assert!(body["id"] != "");
        assert!(body["owner"] == "1");
        assert!(body["url"] == "http://link");
        assert!(body["title"] == "");
        assert!(body["description"] == "");
        assert!(body["created_at"] != "");
        assert!(body["updated_at"] == "");
    }

    #[tokio::test]
    async fn test_post_links_service_error() {
        let mut mock_links_service = MockService::new();
        let mock_links_repo = MockRepository::new();
        let request = PostLinkRequest::new("1", "http://link");

        mock_links_service
            .expect_post()
            .times(1)
            .returning(|_, _| Err(AppError::TestError));

        let app_state = AppState::new(Arc::new(mock_links_service), Arc::new(mock_links_repo));

        let response = post(State(app_state), Json(request)).await;

        let (parts, body) = response.into_response().into_parts();
        assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, parts.status);

        let body = body.collect().await.unwrap().to_bytes();
        let body = std::str::from_utf8(&body).unwrap();
        assert!(body.contains("test error"));
    }

    #[tokio::test]
    async fn test_get_link_not_found() {
        let mut mock_links_service = MockService::new();
        let mock_links_repo = MockRepository::new();
        mock_links_service
            .expect_get()
            .times(1)
            .returning(|_, _| Err(AppError::ItemNotFound));

        let app_state = AppState::new(Arc::new(mock_links_service), Arc::new(mock_links_repo));

        let response = get(State(app_state), Path("1111".to_string())).await;

        let (parts, body) = response.into_response().into_parts();
        assert_eq!(StatusCode::NOT_FOUND, parts.status);

        let body = body.collect().await.unwrap().to_bytes();
        let body = std::str::from_utf8(&body).unwrap();
        assert!(body.contains("item not found"));
    }

    #[tokio::test]
    async fn test_get_link_found() {
        let mut mock_links_service = MockService::new();
        let mock_links_repo = MockRepository::new();
        let item = LinkItem::new("1", "http://link");

        mock_links_service
            .expect_get()
            .times(1)
            .returning(move |_, _| Ok(item.clone()));

        let app_state = AppState::new(Arc::new(mock_links_service), Arc::new(mock_links_repo));

        let response = get(State(app_state), Path("1111".to_string())).await;

        let (parts, body) = response.into_response().into_parts();
        assert_eq!(StatusCode::OK, parts.status);

        let body = body.collect().await.unwrap().to_bytes();
        let body = std::str::from_utf8(&body).unwrap();
        assert!(body.contains("http://link"));
    }

    #[tokio::test]
    async fn test_put_links() {
        let mut mock_links_service = MockService::new();
        let mock_links_repo = MockRepository::new();
        let request = PutLinkRequest::new("1", "http://link");
        let item: LinkItem = request.clone().into();

        mock_links_service
            .expect_put()
            .times(1)
            .returning(move |_, _, _| Ok(item.clone()));

        let app_state = AppState::new(Arc::new(mock_links_service), Arc::new(mock_links_repo));

        let response = put(State(app_state), Path("1111".to_string()), Json(request)).await;

        let (parts, body) = response.into_response().into_parts();
        assert_eq!(StatusCode::OK, parts.status);

        let body = body.collect().await.unwrap().to_bytes();
        let body = std::str::from_utf8(&body).unwrap();
        let body: Value = serde_json::from_str(body).unwrap();

        assert!(body["owner"] == "1");
        assert!(body["url"] == "http://link");
        assert!(body["title"] == "");
        assert!(body["description"] == "");
        assert!(body["created_at"] == "");
        assert!(body["updated_at"] != "");
    }

    #[tokio::test]
    async fn test_put_links_service_error() {
        let mut mock_links_service = MockService::new();
        let mock_links_repo = MockRepository::new();
        let request = PutLinkRequest::new("1", "http://link");

        mock_links_service
            .expect_put()
            .times(1)
            .returning(|_, _, _| Err(AppError::TestError));

        let app_state = AppState::new(Arc::new(mock_links_service), Arc::new(mock_links_repo));

        let response = put(State(app_state), Path("1111".to_string()), Json(request)).await;

        let (parts, body) = response.into_response().into_parts();
        assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, parts.status);

        let body = body.collect().await.unwrap().to_bytes();
        let body = std::str::from_utf8(&body).unwrap();
        assert!(body.contains("test error"));
    }

    #[tokio::test]
    async fn test_delete_links_not_found() {
        let mut mock_links_service = MockService::new();
        let mock_links_repo = MockRepository::new();
        mock_links_service
            .expect_delete()
            .times(1)
            .returning(|_, _| Err(AppError::ItemNotFound));

        let app_state = AppState::new(Arc::new(mock_links_service), Arc::new(mock_links_repo));

        let response = delete(State(app_state), Path("1111".to_string())).await;

        let (parts, body) = response.into_response().into_parts();
        assert_eq!(StatusCode::NOT_FOUND, parts.status);

        let body = body.collect().await.unwrap().to_bytes();
        let body = std::str::from_utf8(&body).unwrap();
        assert!(body.contains("item not found"));
    }

    #[tokio::test]
    async fn test_delete_links_found() {
        let mut mock_links_service = MockService::new();
        let mock_links_repo = MockRepository::new();
        mock_links_service
            .expect_delete()
            .times(1)
            .returning(move |_, _| Ok(()));

        let app_state = AppState::new(Arc::new(mock_links_service), Arc::new(mock_links_repo));

        let response = delete(State(app_state), Path("1111".to_string())).await;

        let (parts, _) = response.into_response().into_parts();
        assert_eq!(StatusCode::NO_CONTENT, parts.status);
    }
}
