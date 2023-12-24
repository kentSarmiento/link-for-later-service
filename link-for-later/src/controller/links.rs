use axum::{
    extract::{self, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing, Json, Router,
};
use validator::Validate;

use crate::{
    state::AppState,
    types::{
        auth::Claims,
        dto::{LinkItemRequest, LinkQueryBuilder},
        entity::LinkItemBuilder,
        AppError,
    },
};

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .nest(
            "/v1",
            Router::new()
                .route("/links", routing::get(list))
                .route("/links", routing::post(post))
                .route("/links/:id", routing::get(get))
                .route("/links/:id", routing::put(put))
                .route("/links/:id", routing::delete(delete)),
        )
        .with_state(state)
}

async fn list(State(app_state): State<AppState>, user: Claims) -> impl IntoResponse {
    let link_query = LinkQueryBuilder::default().owner(user.id()).build();
    match app_state
        .links_service()
        .search(Box::new(app_state.links_repo().clone()), &link_query)
        .await
    {
        Ok(list) => Json(list).into_response(),
        Err(e) => {
            tracing::error!("Error: {}", e);
            e.into_response()
        }
    }
}

async fn post(
    State(app_state): State<AppState>,
    user: Claims,
    Json(payload): extract::Json<LinkItemRequest>,
) -> impl IntoResponse {
    match payload.validate() {
        Ok(()) => {}
        Err(e) => {
            tracing::error!("Error: {}", e);
            return AppError::InvalidUrl.into_response();
        }
    }

    let link_item = LinkItemBuilder::default()
        .owner(user.id())
        .url(payload.url())
        .title(payload.title())
        .description(payload.description())
        .build();
    match app_state
        .links_service()
        .create(Box::new(app_state.links_repo().clone()), &link_item)
        .await
    {
        Ok(link) => (StatusCode::CREATED, Json(link)).into_response(),
        Err(e) => {
            tracing::error!("Error: {}", e);
            e.into_response()
        }
    }
}

async fn get(
    State(app_state): State<AppState>,
    user: Claims,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let link_query = LinkQueryBuilder::new(&id, user.id()).build();
    match app_state
        .links_service()
        .get(Box::new(app_state.links_repo().clone()), &link_query)
        .await
    {
        Ok(link) => Json(link).into_response(),
        Err(e) => {
            tracing::error!("Error: {}", e);
            e.into_response()
        }
    }
}

async fn put(
    State(app_state): State<AppState>,
    user: Claims,
    Path(id): Path<String>,
    Json(payload): extract::Json<LinkItemRequest>,
) -> impl IntoResponse {
    match payload.validate() {
        Ok(()) => {}
        Err(e) => {
            tracing::error!("Error: {}", e);
            return AppError::InvalidUrl.into_response();
        }
    }

    let link_item = LinkItemBuilder::new(payload.url())
        .id(&id)
        .owner(user.id())
        .title(payload.title())
        .description(payload.description())
        .build();
    match app_state
        .links_service()
        .update(Box::new(app_state.links_repo().clone()), &id, &link_item)
        .await
    {
        Ok(link) => Json(link).into_response(),
        Err(e) => {
            tracing::error!("Error: {}", e);
            e.into_response()
        }
    }
}

async fn delete(
    State(app_state): State<AppState>,
    user: Claims,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let link_item = LinkItemBuilder::default().id(&id).owner(user.id()).build();
    match app_state
        .links_service()
        .delete(Box::new(app_state.links_repo().clone()), &link_item)
        .await
    {
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

    use axum::{extract::State, http::StatusCode};
    use http_body_util::BodyExt;
    use serde_json::json;
    use tracing_test::traced_test;

    use crate::{
        repository::{MockLinks as MockLinksRepo, MockUsers as MockUsersRepo},
        service::{
            DynLinks as DynLinksService, MockLinks as MockLinksService,
            MockUsers as MockUsersService,
        },
        state::AppState,
        types::{auth::Claims, entity::LinkItem},
    };

    use super::*;

    #[traced_test]
    #[tokio::test]
    async fn test_get_links_empty() {
        let repo_query = LinkQueryBuilder::default().owner("user-id").build();

        let mut mock_links_service = MockLinksService::new();
        mock_links_service
            .expect_search()
            .withf(move |_, query| query == &repo_query)
            .times(1)
            .returning(|_, _| Ok(vec![]));

        let app_state = AppStateBuilder::new(Arc::new(mock_links_service)).build();
        let response = list(State(app_state), Claims::new("user-id", 0, 0)).await;

        let (parts, body) = response.into_response().into_parts();
        assert_eq!(StatusCode::OK, parts.status);

        let body = body.collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"[]");
    }

    #[traced_test]
    #[tokio::test]
    async fn test_get_links_non_empty() {
        let repo_query = LinkQueryBuilder::default().owner("user-id").build();
        let item = LinkItemBuilder::new("http://link")
            .id("1")
            .owner("user-id")
            .build();

        let mut mock_links_service = MockLinksService::new();
        mock_links_service
            .expect_search()
            .withf(move |_, query| query == &repo_query)
            .times(1)
            .returning(move |_, _| Ok(vec![item.clone()]));

        let app_state = AppStateBuilder::new(Arc::new(mock_links_service)).build();
        let response = list(State(app_state), Claims::new("user-id", 0, 0)).await;

        let (parts, body) = response.into_response().into_parts();
        assert_eq!(StatusCode::OK, parts.status);

        let body = body.collect().await.unwrap().to_bytes();
        let body = std::str::from_utf8(&body).unwrap();
        let body: Vec<LinkItem> = serde_json::from_str(body).unwrap();
        assert!(body[0].id() == "1");
        assert!(body[0].owner() == "user-id");
        assert!(body[0].url() == "http://link");
    }

    #[traced_test]
    #[tokio::test]
    async fn test_get_links_service_error() {
        let repo_query = LinkQueryBuilder::default().owner("user-id").build();

        let mut mock_links_service = MockLinksService::new();
        mock_links_service
            .expect_search()
            .withf(move |_, query| query == &repo_query)
            .times(1)
            .returning(|_, _| Err(AppError::ServerError));

        let app_state = AppStateBuilder::new(Arc::new(mock_links_service)).build();
        let response = list(State(app_state), Claims::new("user-id", 0, 0)).await;

        let (parts, body) = response.into_response().into_parts();
        assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, parts.status);

        let body = body.collect().await.unwrap().to_bytes();
        let body = std::str::from_utf8(&body).unwrap();
        assert_eq!(body, json!({"error": "server error"}).to_string());
    }

    #[traced_test]
    #[tokio::test]
    async fn test_post_link() {
        let request = LinkItemRequest::new("http://link");
        let item_to_create = LinkItemBuilder::new("http://link").owner("user-id").build();
        let created_item = LinkItemBuilder::new("http://link")
            .id("1")
            .owner("user-id")
            .build();

        let mut mock_links_service = MockLinksService::new();
        mock_links_service
            .expect_create()
            .withf(move |_, item| item == &item_to_create)
            .times(1)
            .returning(move |_, _| Ok(created_item.clone()));

        let app_state = AppStateBuilder::new(Arc::new(mock_links_service)).build();
        let response = post(
            State(app_state),
            Claims::new("user-id", 0, 0),
            Json(request),
        )
        .await;

        let (parts, body) = response.into_response().into_parts();
        assert_eq!(StatusCode::CREATED, parts.status);

        let body = body.collect().await.unwrap().to_bytes();
        let body = std::str::from_utf8(&body).unwrap();
        let body: LinkItem = serde_json::from_str(body).unwrap();
        assert!(body.id() == "1");
        assert!(body.owner() == "user-id");
        assert!(body.url() == "http://link");
    }

    #[traced_test]
    #[tokio::test]
    async fn test_post_link_invalid_url() {
        let request = LinkItemRequest::new("invalid-link");

        let mut mock_links_service = MockLinksService::new();
        mock_links_service.expect_create().times(0);

        let app_state = AppStateBuilder::new(Arc::new(mock_links_service)).build();
        let response = post(
            State(app_state),
            Claims::new("user-id", 0, 0),
            Json(request),
        )
        .await;

        let (parts, body) = response.into_response().into_parts();
        assert_eq!(StatusCode::BAD_REQUEST, parts.status);

        let body = body.collect().await.unwrap().to_bytes();
        let body = std::str::from_utf8(&body).unwrap();
        assert_eq!(body, json!({"error": "invalid url"}).to_string());
    }

    #[traced_test]
    #[tokio::test]
    async fn test_post_link_service_error() {
        let request = LinkItemRequest::new("http://link");
        let item_to_create = LinkItemBuilder::new("http://link").owner("user-id").build();

        let mut mock_links_service = MockLinksService::new();
        mock_links_service
            .expect_create()
            .withf(move |_, item| item == &item_to_create)
            .times(1)
            .returning(|_, _| Err(AppError::ServerError));

        let app_state = AppStateBuilder::new(Arc::new(mock_links_service)).build();
        let response = post(
            State(app_state),
            Claims::new("user-id", 0, 0),
            Json(request),
        )
        .await;

        let (parts, body) = response.into_response().into_parts();
        assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, parts.status);

        let body = body.collect().await.unwrap().to_bytes();
        let body = std::str::from_utf8(&body).unwrap();
        assert_eq!(body, json!({"error": "server error"}).to_string());
    }

    #[traced_test]
    #[tokio::test]
    async fn test_get_link() {
        let repo_query = LinkQueryBuilder::new("1", "user-id").build();
        let retrieved_item = LinkItemBuilder::new("http://link")
            .id("1")
            .owner("user-id")
            .build();

        let mut mock_links_service = MockLinksService::new();
        mock_links_service
            .expect_get()
            .withf(move |_, query| query == &repo_query)
            .times(1)
            .returning(move |_, _| Ok(retrieved_item.clone()));

        let app_state = AppStateBuilder::new(Arc::new(mock_links_service)).build();
        let response = get(
            State(app_state),
            Claims::new("user-id", 0, 0),
            Path(String::from("1")),
        )
        .await;

        let (parts, body) = response.into_response().into_parts();
        assert_eq!(StatusCode::OK, parts.status);

        let body = body.collect().await.unwrap().to_bytes();
        let body = std::str::from_utf8(&body).unwrap();
        let body: LinkItem = serde_json::from_str(body).unwrap();
        assert!(body.id() == "1");
        assert!(body.owner() == "user-id");
        assert!(body.url() == "http://link");
    }

    #[traced_test]
    #[tokio::test]
    async fn test_get_link_service_error() {
        let repo_query = LinkQueryBuilder::new("1", "user-id").build();

        let mut mock_links_service = MockLinksService::new();
        mock_links_service
            .expect_get()
            .withf(move |_, query| query == &repo_query)
            .times(1)
            .returning(|_, _| Err(AppError::ServerError));

        let app_state = AppStateBuilder::new(Arc::new(mock_links_service)).build();
        let response = get(
            State(app_state),
            Claims::new("user-id", 0, 0),
            Path(String::from("1")),
        )
        .await;

        let (parts, body) = response.into_response().into_parts();
        assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, parts.status);

        let body = body.collect().await.unwrap().to_bytes();
        let body = std::str::from_utf8(&body).unwrap();
        assert_eq!(body, json!({"error": "server error"}).to_string());
    }

    #[traced_test]
    #[tokio::test]
    async fn test_put_link() {
        let request = LinkItemRequest::new("http://link");
        let item_to_update = LinkItemBuilder::new("http://link")
            .id("1")
            .owner("user-id")
            .build();
        let updated_item = LinkItemBuilder::new("http://link")
            .id("1")
            .owner("user-id")
            .build();

        let mut mock_links_service = MockLinksService::new();
        mock_links_service
            .expect_update()
            .withf(move |_, id, item| id == "1" && item == &item_to_update)
            .times(1)
            .returning(move |_, _, _| Ok(updated_item.clone()));

        let app_state = AppStateBuilder::new(Arc::new(mock_links_service)).build();
        let response = put(
            State(app_state),
            Claims::new("user-id", 0, 0),
            Path(String::from("1")),
            Json(request),
        )
        .await;

        let (parts, body) = response.into_response().into_parts();
        assert_eq!(StatusCode::OK, parts.status);

        let body = body.collect().await.unwrap().to_bytes();
        let body = std::str::from_utf8(&body).unwrap();
        let body: LinkItem = serde_json::from_str(body).unwrap();
        assert!(body.id() == "1");
        assert!(body.owner() == "user-id");
        assert!(body.url() == "http://link");
    }

    #[traced_test]
    #[tokio::test]
    async fn test_put_link_invalid_url() {
        let request = LinkItemRequest::new("invalid-link");

        let mut mock_links_service = MockLinksService::new();
        mock_links_service.expect_update().times(0);

        let app_state = AppStateBuilder::new(Arc::new(mock_links_service)).build();
        let response = put(
            State(app_state),
            Claims::new("user-id", 0, 0),
            Path(String::from("1")),
            Json(request),
        )
        .await;

        let (parts, body) = response.into_response().into_parts();
        assert_eq!(StatusCode::BAD_REQUEST, parts.status);

        let body = body.collect().await.unwrap().to_bytes();
        let body = std::str::from_utf8(&body).unwrap();
        assert_eq!(body, json!({"error": "invalid url"}).to_string());
    }

    #[traced_test]
    #[tokio::test]
    async fn test_put_link_service_error() {
        let request = LinkItemRequest::new("http://link");
        let item_to_update = LinkItemBuilder::new("http://link")
            .id("1")
            .owner("user-id")
            .build();

        let mut mock_links_service = MockLinksService::new();
        mock_links_service
            .expect_update()
            .withf(move |_, id, item| id == "1" && item == &item_to_update)
            .times(1)
            .returning(|_, _, _| Err(AppError::ServerError));

        let app_state = AppStateBuilder::new(Arc::new(mock_links_service)).build();
        let response = put(
            State(app_state),
            Claims::new("user-id", 0, 0),
            Path(String::from("1")),
            Json(request),
        )
        .await;

        let (parts, body) = response.into_response().into_parts();
        assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, parts.status);

        let body = body.collect().await.unwrap().to_bytes();
        let body = std::str::from_utf8(&body).unwrap();
        assert_eq!(body, json!({"error": "server error"}).to_string());
    }

    #[traced_test]
    #[tokio::test]
    async fn test_delete_link() {
        let item_to_delete = LinkItemBuilder::default().id("1").owner("user-id").build();

        let mut mock_links_service = MockLinksService::new();
        mock_links_service
            .expect_delete()
            .withf(move |_, item| item == &item_to_delete)
            .times(1)
            .returning(move |_, _| Ok(()));

        let app_state = AppStateBuilder::new(Arc::new(mock_links_service)).build();
        let response = delete(
            State(app_state),
            Claims::new("user-id", 0, 0),
            Path(String::from("1")),
        )
        .await;

        let (parts, _) = response.into_response().into_parts();
        assert_eq!(StatusCode::NO_CONTENT, parts.status);
    }

    #[traced_test]
    #[tokio::test]
    async fn test_delete_link_service_error() {
        let item_to_delete = LinkItemBuilder::default().id("1").owner("user-id").build();

        let mut mock_links_service = MockLinksService::new();
        mock_links_service
            .expect_delete()
            .withf(move |_, item| item == &item_to_delete)
            .times(1)
            .returning(|_, _| Err(AppError::ServerError));

        let app_state = AppStateBuilder::new(Arc::new(mock_links_service)).build();
        let response = delete(
            State(app_state),
            Claims::new("user-id", 0, 0),
            Path(String::from("1")),
        )
        .await;

        let (parts, body) = response.into_response().into_parts();
        assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, parts.status);

        let body = body.collect().await.unwrap().to_bytes();
        let body = std::str::from_utf8(&body).unwrap();
        assert_eq!(body, json!({"error": "server error"}).to_string());
    }

    struct AppStateBuilder {
        links_service: DynLinksService,
    }

    impl AppStateBuilder {
        fn new(links_service: DynLinksService) -> Self {
            Self { links_service }
        }

        fn build(self) -> AppState {
            AppState::new(
                self.links_service,
                Arc::new(MockUsersService::new()),
                Arc::new(MockLinksRepo::new()),
                Arc::new(MockUsersRepo::new()),
            )
        }
    }
}
