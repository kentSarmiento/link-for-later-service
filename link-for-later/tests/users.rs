use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt;

mod app;
mod auth;
mod entity;
mod repository;

#[tokio::test]
async fn test_register_user() {
    repository::setup();

    let request = r#"{
        "email": "user@test.com",
        "password": "test"
    }"#;

    let response = app::new()
        .await
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/users/register")
                .header("Content-Type", "application/json")
                .body(Body::from(request))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], b"");

    let db_count = repository::count_users().await;
    assert!(db_count == 1);

    let db_item = repository::get_user("user@test.com").await;
    assert!(db_item.email == "user@test.com");
    assert!(db_item.password == "test");
}

#[tokio::test]
async fn test_register_user_invalid_email() {
    repository::setup();

    let request = r#"{
        "email": "user",
        "password": "test"
    }"#;

    let response = app::new()
        .await
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/users/register")
                .header("Content-Type", "application/json")
                .body(Body::from(request))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body = std::str::from_utf8(&body).unwrap();
    assert_eq!(body, json!({"error": "invalid email"}).to_string());

    let db_count = repository::count_users().await;
    assert!(db_count == 0);
}

#[tokio::test]
async fn test_register_user_already_registered() {
    repository::setup();

    repository::add_user("user@test.com", "test").await;
    let request = r#"{
        "email": "user@test.com",
        "password": "test"
    }"#;

    let response = app::new()
        .await
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/users/register")
                .header("Content-Type", "application/json")
                .body(Body::from(request))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body = std::str::from_utf8(&body).unwrap();
    assert_eq!(body, json!({"error": "user already regisered"}).to_string());

    let db_count = repository::count_users().await;
    assert!(db_count == 1);
}

#[tokio::test]
async fn test_login_user() {
    repository::setup();

    repository::add_user("user@test.com", "test").await;
    let request = r#"{
        "email": "user@test.com",
        "password": "test"
    }"#;

    let response = app::new()
        .await
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/users/login")
                .header("Content-Type", "application/json")
                .body(Body::from(request))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body = std::str::from_utf8(&body).unwrap();
    let body: Value = serde_json::from_str(body).unwrap();
    assert!(!body["token"].to_string().is_empty());
}

#[tokio::test]
async fn test_login_user_invalid_email() {
    repository::setup();

    let request = r#"{
        "email": "user",
        "password": "test"
    }"#;

    let response = app::new()
        .await
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/users/login")
                .header("Content-Type", "application/json")
                .body(Body::from(request))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body = std::str::from_utf8(&body).unwrap();
    assert_eq!(body, json!({"error": "invalid email"}).to_string());
}

#[tokio::test]
async fn test_login_user_not_found() {
    repository::setup();

    repository::add_user("user@test.com", "test").await;
    let request = r#"{
        "email": "user2@test.com",
        "password": "test"
    }"#;

    let response = app::new()
        .await
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/users/login")
                .header("Content-Type", "application/json")
                .body(Body::from(request))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body = std::str::from_utf8(&body).unwrap();
    assert_eq!(body, json!({"error": "user not found"}).to_string());
}
