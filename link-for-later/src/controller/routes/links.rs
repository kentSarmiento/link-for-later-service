use axum::{
    extract::{self, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing, Json, Router,
};
use validator::Validate;

use crate::types::{
    AppError, AppState, Claims, LinkItemBuilder, LinkItemRequest, LinkQueryBuilder,
};

pub fn router(state: AppState) -> Router<AppState> {
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
    let query = LinkQueryBuilder::default()
        .user(user.id())
        .is_from_admin(user.is_admin())
        .build();
    match app_state
        .links_service()
        .search(Box::new(app_state.links_repo().clone()), &query)
        .await
    {
        Ok(list) => Json(list).into_response(),
        Err(e) => e.into_response(),
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
            return AppError::Validation(format!("post_link() {e:?}")).into_response();
        }
    }

    let item = LinkItemBuilder::default()
        .owner(user.id())
        .url(payload.url())
        .title(payload.title())
        .description(payload.description())
        .build();
    match app_state
        .links_service()
        .create(
            Box::new(app_state.analysis_service().clone()),
            Box::new(app_state.links_repo().clone()),
            &item,
        )
        .await
    {
        Ok(item) => (StatusCode::CREATED, Json(item)).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn get(
    State(app_state): State<AppState>,
    user: Claims,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let query = LinkQueryBuilder::new(&id, user.id())
        .is_from_admin(user.is_admin())
        .build();
    match app_state
        .links_service()
        .get(Box::new(app_state.links_repo().clone()), &query)
        .await
    {
        Ok(item) => Json(item).into_response(),
        Err(e) => e.into_response(),
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
            return AppError::Validation(format!("put_link() {e:?}")).into_response();
        }
    }

    let query = LinkQueryBuilder::new(&id, user.id())
        .is_from_admin(user.is_admin())
        .build();
    let item = LinkItemBuilder::new(payload.url())
        .id(&id)
        .title(payload.title())
        .description(payload.description())
        .word_count(payload.word_count())
        .reading_time(payload.reading_time())
        .summary(payload.summary())
        .label(payload.label())
        .build();
    match app_state
        .links_service()
        .update(
            Box::new(app_state.analysis_service().clone()),
            Box::new(app_state.links_repo().clone()),
            &query,
            &item,
        )
        .await
    {
        Ok(item) => Json(item).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn delete(
    State(app_state): State<AppState>,
    user: Claims,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let query = LinkQueryBuilder::new(&id, user.id())
        .is_from_admin(user.is_admin())
        .build();
    match app_state
        .links_service()
        .delete(Box::new(app_state.links_repo().clone()), &query)
        .await
    {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => e.into_response(),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use axum::{extract::State, http::StatusCode};
    use http_body_util::BodyExt;
    use rstest::rstest;
    use serde_json::json;

    use crate::{
        repository::{MockLinks as MockLinksRepo, MockUsers as MockUsersRepo},
        service::DynLinks as DynLinksService,
        service::{
            MockAnalysis as MockAnalysisService, MockLinks as MockLinksService,
            MockUsers as MockUsersService,
        },
        types::LinkItem,
    };

    use super::*;

    #[rstest]
    #[case(true, "admin")]
    #[case(false, "user")]
    #[tokio::test]
    async fn test_get_links_empty(#[case] is_admin: bool, #[case] user: &str) {
        let search_query = LinkQueryBuilder::default()
            .user(user)
            .is_from_admin(is_admin)
            .build();

        let mut mock_links_service = MockLinksService::new();
        mock_links_service
            .expect_search()
            .withf(move |_, query| query == &search_query)
            .times(1)
            .returning(|_, _| Ok(vec![]));

        let app_state = AppStateBuilder::new(Arc::new(mock_links_service)).build();
        let response = list(State(app_state), Claims::new(user, is_admin, 0, 0)).await;

        let (parts, body) = response.into_response().into_parts();
        assert_eq!(StatusCode::OK, parts.status);

        let body = body.collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"[]");
    }

    #[rstest]
    #[case(true, "admin")]
    #[case(false, "user")]
    #[tokio::test]
    async fn test_get_links_non_empty(#[case] is_admin: bool, #[case] user: &str) {
        let search_query = LinkQueryBuilder::default()
            .user(user)
            .is_from_admin(is_admin)
            .build();
        let item = LinkItemBuilder::new("http://link")
            .id("1")
            .owner("user")
            .build();

        let mut mock_links_service = MockLinksService::new();
        mock_links_service
            .expect_search()
            .withf(move |_, query| query == &search_query)
            .times(1)
            .returning(move |_, _| Ok(vec![item.clone()]));

        let app_state = AppStateBuilder::new(Arc::new(mock_links_service)).build();
        let response = list(State(app_state), Claims::new(user, is_admin, 0, 0)).await;

        let (parts, body) = response.into_response().into_parts();
        assert_eq!(StatusCode::OK, parts.status);

        let body = body.collect().await.unwrap().to_bytes();
        let body = std::str::from_utf8(&body).unwrap();
        let body: Vec<LinkItem> = serde_json::from_str(body).unwrap();
        assert!(body[0].id() == "1");
        assert!(body[0].owner() == "user");
        assert!(body[0].url() == "http://link");
    }

    #[rstest]
    #[case(true, "admin")]
    #[case(false, "user")]
    #[tokio::test]
    async fn test_get_links_service_error(#[case] is_admin: bool, #[case] user: &str) {
        let search_query = LinkQueryBuilder::default()
            .user(user)
            .is_from_admin(is_admin)
            .build();

        let mut mock_links_service = MockLinksService::new();
        mock_links_service
            .expect_search()
            .withf(move |_, query| query == &search_query)
            .times(1)
            .returning(|_, _| Err(AppError::Test));

        let app_state = AppStateBuilder::new(Arc::new(mock_links_service)).build();
        let response = list(State(app_state), Claims::new(user, is_admin, 0, 0)).await;

        let (parts, body) = response.into_response().into_parts();
        assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, parts.status);

        let body = body.collect().await.unwrap().to_bytes();
        let body = std::str::from_utf8(&body).unwrap();
        assert_eq!(body, json!({"error": "test error"}).to_string());
    }

    #[rstest]
    #[tokio::test]
    async fn test_post_link(#[values(true, false)] is_admin: bool) {
        let request = LinkItemRequest::new("http://link");
        let item_to_create = LinkItemBuilder::new("http://link").owner("user").build();
        let created_item = LinkItemBuilder::new("http://link")
            .id("1")
            .owner("user")
            .build();

        let mut mock_links_service = MockLinksService::new();
        mock_links_service
            .expect_create()
            .withf(move |_, _, item| item == &item_to_create)
            .times(1)
            .returning(move |_, _, _| Ok(created_item.clone()));

        let app_state = AppStateBuilder::new(Arc::new(mock_links_service)).build();
        let response = post(
            State(app_state),
            Claims::new("user", is_admin, 0, 0),
            Json(request),
        )
        .await;

        let (parts, body) = response.into_response().into_parts();
        assert_eq!(StatusCode::CREATED, parts.status);

        let body = body.collect().await.unwrap().to_bytes();
        let body = std::str::from_utf8(&body).unwrap();
        let body: LinkItem = serde_json::from_str(body).unwrap();
        assert!(body.id() == "1");
        assert!(body.owner() == "user");
        assert!(body.url() == "http://link");
    }

    #[rstest]
    #[tokio::test]
    async fn test_post_link_invalid_url(#[values(true, false)] is_admin: bool) {
        let request = LinkItemRequest::new("invalid-link");

        let mut mock_links_service = MockLinksService::new();
        mock_links_service.expect_create().times(0);

        let app_state = AppStateBuilder::new(Arc::new(mock_links_service)).build();
        let response = post(
            State(app_state),
            Claims::new("user", is_admin, 0, 0),
            Json(request),
        )
        .await;

        let (parts, body) = response.into_response().into_parts();
        assert_eq!(StatusCode::BAD_REQUEST, parts.status);

        let body = body.collect().await.unwrap().to_bytes();
        let body = std::str::from_utf8(&body).unwrap();
        assert_eq!(body, json!({"error": "invalid request"}).to_string());
    }

    #[rstest]
    #[tokio::test]
    async fn test_post_link_service_error(#[values(true, false)] is_admin: bool) {
        let request = LinkItemRequest::new("http://link");
        let item_to_create = LinkItemBuilder::new("http://link").owner("user").build();

        let mut mock_links_service = MockLinksService::new();
        mock_links_service
            .expect_create()
            .withf(move |_, _, item| item == &item_to_create)
            .times(1)
            .returning(|_, _, _| Err(AppError::Test));

        let app_state = AppStateBuilder::new(Arc::new(mock_links_service)).build();
        let response = post(
            State(app_state),
            Claims::new("user", is_admin, 0, 0),
            Json(request),
        )
        .await;

        let (parts, body) = response.into_response().into_parts();
        assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, parts.status);

        let body = body.collect().await.unwrap().to_bytes();
        let body = std::str::from_utf8(&body).unwrap();
        assert_eq!(body, json!({"error": "test error"}).to_string());
    }

    #[rstest]
    #[case(true, "admin")]
    #[case(false, "user")]
    #[tokio::test]
    async fn test_get_link(#[case] is_admin: bool, #[case] user: &str) {
        let get_query = LinkQueryBuilder::new("1", user)
            .is_from_admin(is_admin)
            .build();
        let item = LinkItemBuilder::new("http://link")
            .id("1")
            .owner("user")
            .build();

        let mut mock_links_service = MockLinksService::new();
        mock_links_service
            .expect_get()
            .withf(move |_, query| query == &get_query)
            .times(1)
            .returning(move |_, _| Ok(item.clone()));

        let app_state = AppStateBuilder::new(Arc::new(mock_links_service)).build();
        let response = get(
            State(app_state),
            Claims::new(user, is_admin, 0, 0),
            Path(String::from("1")),
        )
        .await;

        let (parts, body) = response.into_response().into_parts();
        assert_eq!(StatusCode::OK, parts.status);

        let body = body.collect().await.unwrap().to_bytes();
        let body = std::str::from_utf8(&body).unwrap();
        let body: LinkItem = serde_json::from_str(body).unwrap();
        assert!(body.id() == "1");
        assert!(body.owner() == "user");
        assert!(body.url() == "http://link");
    }

    #[rstest]
    #[case(true, "admin")]
    #[case(false, "user")]
    #[tokio::test]
    async fn test_get_link_service_error(#[case] is_admin: bool, #[case] user: &str) {
        let get_query = LinkQueryBuilder::new("1", user)
            .is_from_admin(is_admin)
            .build();

        let mut mock_links_service = MockLinksService::new();
        mock_links_service
            .expect_get()
            .withf(move |_, query| query == &get_query)
            .times(1)
            .returning(|_, _| Err(AppError::Test));

        let app_state = AppStateBuilder::new(Arc::new(mock_links_service)).build();
        let response = get(
            State(app_state),
            Claims::new(user, is_admin, 0, 0),
            Path(String::from("1")),
        )
        .await;

        let (parts, body) = response.into_response().into_parts();
        assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, parts.status);

        let body = body.collect().await.unwrap().to_bytes();
        let body = std::str::from_utf8(&body).unwrap();
        assert_eq!(body, json!({"error": "test error"}).to_string());
    }

    #[rstest]
    #[case(true, "admin")]
    #[case(false, "user")]
    #[tokio::test]
    async fn test_put_link(#[case] is_admin: bool, #[case] user: &str) {
        let request = LinkItemRequest::new("http://link");
        let update_query = LinkQueryBuilder::new("1", user)
            .is_from_admin(is_admin)
            .build();
        let item_to_update = LinkItemBuilder::new("http://link").id("1").build();
        let updated_item = LinkItemBuilder::new("http://link").id("1").build();

        let mut mock_links_service = MockLinksService::new();
        mock_links_service
            .expect_update()
            .withf(move |_, _, query, item| query == &update_query && item == &item_to_update)
            .times(1)
            .returning(move |_, _, _, _| Ok(updated_item.clone()));

        let app_state = AppStateBuilder::new(Arc::new(mock_links_service)).build();
        let response = put(
            State(app_state),
            Claims::new(user, is_admin, 0, 0),
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
        assert!(body.url() == "http://link");
    }

    #[rstest]
    #[tokio::test]
    async fn test_put_link_invalid_url(#[values(true, false)] is_admin: bool) {
        let request = LinkItemRequest::new("invalid-link");

        let mut mock_links_service = MockLinksService::new();
        mock_links_service.expect_update().times(0);

        let app_state = AppStateBuilder::new(Arc::new(mock_links_service)).build();
        let response = put(
            State(app_state),
            Claims::new("user", is_admin, 0, 0),
            Path(String::from("1")),
            Json(request),
        )
        .await;

        let (parts, body) = response.into_response().into_parts();
        assert_eq!(StatusCode::BAD_REQUEST, parts.status);

        let body = body.collect().await.unwrap().to_bytes();
        let body = std::str::from_utf8(&body).unwrap();
        assert_eq!(body, json!({"error": "invalid request"}).to_string());
    }

    #[rstest]
    #[case(true, "admin")]
    #[case(false, "user")]
    #[tokio::test]
    async fn test_put_link_service_error(#[case] is_admin: bool, #[case] user: &str) {
        let request = LinkItemRequest::new("http://link");
        let update_query = LinkQueryBuilder::new("1", user)
            .is_from_admin(is_admin)
            .build();
        let item_to_update = LinkItemBuilder::new("http://link").id("1").build();

        let mut mock_links_service = MockLinksService::new();
        mock_links_service
            .expect_update()
            .withf(move |_, _, query, item| query == &update_query && item == &item_to_update)
            .times(1)
            .returning(|_, _, _, _| Err(AppError::Test));

        let app_state = AppStateBuilder::new(Arc::new(mock_links_service)).build();
        let response = put(
            State(app_state),
            Claims::new(user, is_admin, 0, 0),
            Path(String::from("1")),
            Json(request),
        )
        .await;

        let (parts, body) = response.into_response().into_parts();
        assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, parts.status);

        let body = body.collect().await.unwrap().to_bytes();
        let body = std::str::from_utf8(&body).unwrap();
        assert_eq!(body, json!({"error": "test error"}).to_string());
    }

    #[rstest]
    #[case(true, "admin")]
    #[case(false, "user")]
    #[tokio::test]
    async fn test_delete_link(#[case] is_admin: bool, #[case] user: &str) {
        let delete_query = LinkQueryBuilder::new("1", user)
            .is_from_admin(is_admin)
            .build();

        let mut mock_links_service = MockLinksService::new();
        mock_links_service
            .expect_delete()
            .withf(move |_, query| query == &delete_query)
            .times(1)
            .returning(move |_, _| Ok(()));

        let app_state = AppStateBuilder::new(Arc::new(mock_links_service)).build();
        let response = delete(
            State(app_state),
            Claims::new(user, is_admin, 0, 0),
            Path(String::from("1")),
        )
        .await;

        let (parts, _) = response.into_response().into_parts();
        assert_eq!(StatusCode::NO_CONTENT, parts.status);
    }

    #[rstest]
    #[case(true, "admin")]
    #[case(false, "user")]
    #[tokio::test]
    async fn test_delete_link_service_error(#[case] is_admin: bool, #[case] user: &str) {
        let delete_query = LinkQueryBuilder::new("1", user)
            .is_from_admin(is_admin)
            .build();

        let mut mock_links_service = MockLinksService::new();
        mock_links_service
            .expect_delete()
            .withf(move |_, query| query == &delete_query)
            .times(1)
            .returning(|_, _| Err(AppError::Test));

        let app_state = AppStateBuilder::new(Arc::new(mock_links_service)).build();
        let response = delete(
            State(app_state),
            Claims::new(user, is_admin, 0, 0),
            Path(String::from("1")),
        )
        .await;

        let (parts, body) = response.into_response().into_parts();
        assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, parts.status);

        let body = body.collect().await.unwrap().to_bytes();
        let body = std::str::from_utf8(&body).unwrap();
        assert_eq!(body, json!({"error": "test error"}).to_string());
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
                Arc::new(MockAnalysisService::new()),
                Arc::new(MockLinksRepo::new()),
                Arc::new(MockUsersRepo::new()),
            )
        }
    }
}
