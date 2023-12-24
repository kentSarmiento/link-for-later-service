use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::json;
use tower::ServiceExt;
use tracing_test::traced_test;

use crate::entity::LinkItem;

mod app;
mod auth;
mod entity;
mod repository;

#[traced_test]
#[tokio::test]
async fn test_get_links_empty() {
    repository::setup();

    let token = auth::generate_token("user@test.com");

    let response = app::new()
        .await
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/v1/links")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], b"[]");
}

#[traced_test]
#[tokio::test]
async fn test_get_links_non_empty() {
    repository::setup();

    let id = repository::add_link("user@test.com", "http://test").await;
    let token = auth::generate_token("user@test.com");

    let response = app::new()
        .await
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/v1/links")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body = std::str::from_utf8(&body).unwrap();
    let body: Vec<LinkItem> = serde_json::from_str(body).unwrap();
    assert!(body.len() == 1);
    assert!(body[0].id == id);
    assert!(body[0].owner == "user@test.com");
    assert!(body[0].url == "http://test");
}

#[traced_test]
#[tokio::test]
async fn test_get_link_item_found() {
    repository::setup();

    let id = repository::add_link("user@test.com", "http://test").await;
    let token = auth::generate_token("user@test.com");

    let response = app::new()
        .await
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/v1/links/{id}"))
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body = std::str::from_utf8(&body).unwrap();
    let body: LinkItem = serde_json::from_str(body).unwrap();
    assert!(body.id == id);
    assert!(body.owner == "user@test.com");
    assert!(body.url == "http://test");
}

#[traced_test]
#[tokio::test]
async fn test_get_link_item_not_found() {
    repository::setup();

    repository::add_link("user@test.com", "http://test").await;
    let token = auth::generate_token("user@test.com");

    let response = app::new()
        .await
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/v1/links/1")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body = std::str::from_utf8(&body).unwrap();
    assert_eq!(body, json!({"error": "link item not found"}).to_string());
}

#[traced_test]
#[tokio::test]
async fn test_post_link() {
    repository::setup();

    let token = auth::generate_token("user@test.com");
    let request = r#"{
        "url": "http://test"
    }"#;

    let response = app::new()
        .await
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/links")
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::from(request))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body = std::str::from_utf8(&body).unwrap();
    let body: LinkItem = serde_json::from_str(body).unwrap();

    assert!(body.owner == "user@test.com");
    assert!(body.url == "http://test");

    let db_count = repository::count_links().await;
    assert!(db_count == 1);

    let db_item = repository::get_link(&body.id).await;
    assert!(db_item.owner == "user@test.com");
    assert!(db_item.url == "http://test");
}

#[traced_test]
#[tokio::test]
async fn test_post_link_invalid_url() {
    repository::setup();

    let token = auth::generate_token("user@test.com");
    let request = r#"{
        "url": "invalid"
    }"#;

    let response = app::new()
        .await
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/links")
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::from(request))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body = std::str::from_utf8(&body).unwrap();
    assert_eq!(body, json!({"error": "invalid url"}).to_string());

    let db_count = repository::count_links().await;
    assert!(db_count == 0);
}

#[traced_test]
#[tokio::test]
async fn test_put_link() {
    repository::setup();

    let id = repository::add_link("user@test.com", "http://test").await;
    let token = auth::generate_token("user@test.com");
    let request = r#"{
        "url": "http://update"
    }"#;

    let response = app::new()
        .await
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/v1/links/{id}"))
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::from(request))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body = std::str::from_utf8(&body).unwrap();
    let body: LinkItem = serde_json::from_str(body).unwrap();

    assert!(body.id == id);
    assert!(body.owner == "user@test.com");
    assert!(body.url == "http://update");

    let db_count = repository::count_links().await;
    assert!(db_count == 1);

    let db_item = repository::get_link(&id).await;
    assert!(db_item.id == id);
    assert!(db_item.owner == "user@test.com");
    assert!(db_item.url == "http://update");
}

#[traced_test]
#[tokio::test]
async fn test_put_link_invalid_url() {
    repository::setup();

    let id = repository::add_link("user@test.com", "http://test").await;
    let token = auth::generate_token("user@test.com");
    let request = r#"{
        "url": "invalid"
    }"#;

    let response = app::new()
        .await
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/v1/links/{id}"))
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::from(request))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body = std::str::from_utf8(&body).unwrap();
    assert_eq!(body, json!({"error": "invalid url"}).to_string());

    let db_count = repository::count_links().await;
    assert!(db_count == 1);
}

#[traced_test]
#[tokio::test]
async fn test_put_link_item_not_found() {
    repository::setup();

    let id = repository::add_link("user@test.com", "http://test").await;
    let token = auth::generate_token("user@test.com");
    let request = r#"{
        "url": "http://update"
    }"#;

    let response = app::new()
        .await
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/v1/links/1")
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::from(request))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body = std::str::from_utf8(&body).unwrap();
    assert_eq!(body, json!({"error": "link item not found"}).to_string());

    let db_count = repository::count_links().await;
    assert!(db_count == 1);

    let db_item = repository::get_link(&id).await;
    assert!(db_item.id == id);
    assert!(db_item.owner == "user@test.com");
    assert!(db_item.url == "http://test"); // not updated
}

#[traced_test]
#[tokio::test]
async fn test_delete_link() {
    repository::setup();

    let id = repository::add_link("user@test.com", "http://test").await;
    let token = auth::generate_token("user@test.com");

    let response = app::new()
        .await
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/v1/links/{id}"))
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], b"");

    let db_count = repository::count_links().await;
    assert!(db_count == 0);
}

#[traced_test]
#[tokio::test]
async fn test_delete_link_item_not_found() {
    repository::setup();

    let id = repository::add_link("user@test.com", "http://test").await;
    let token = auth::generate_token("user@test.com");

    let response = app::new()
        .await
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/v1/links/1")
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body = std::str::from_utf8(&body).unwrap();
    assert_eq!(body, json!({"error": "link item not found"}).to_string());

    let db_count = repository::count_links().await;
    assert!(db_count == 1);

    let db_item = repository::get_link(&id).await;
    assert!(db_item.id == id);
    assert!(db_item.owner == "user@test.com");
    assert!(db_item.url == "http://test"); // not updated
}

#[traced_test]
#[tokio::test]
async fn test_unauthorized_access_to_links_no_token() {
    repository::setup();

    let response = app::new()
        .await
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/v1/links")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body = std::str::from_utf8(&body).unwrap();
    assert_eq!(
        body,
        json!({"error": "invalid authorization token"}).to_string()
    );
}

#[traced_test]
#[tokio::test]
async fn test_unauthorized_access_to_links_invalid_token() {
    repository::setup();

    let response = app::new()
        .await
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/v1/links")
                .header("Authorization", "Bearer invalid")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body = std::str::from_utf8(&body).unwrap();
    assert_eq!(
        body,
        json!({"error": "invalid authorization token"}).to_string()
    );
}
