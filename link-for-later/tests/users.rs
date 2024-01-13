#![allow(dead_code)]

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use rstest::rstest;
use serde_json::{json, Value};
use tower::ServiceExt;

use crate::repository::DatabaseType;

mod app;
mod auth;
mod repository;

#[rstest]
#[case(true, "admin@test.com")]
#[case(false, "user@test.com")]
#[tokio::test]
async fn test_register_user(
    #[values(DatabaseType::MongoDb)] db_type: DatabaseType,
    #[case] is_admin: bool,
    #[case] user: &str,
) {
    let repository = repository::new(&db_type);

    let request = format!(
        r#"{{
        "email": "{}",
        "password": "test",
        "admin": {}
    }}"#,
        user, is_admin
    );

    let response = app::new(&db_type)
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

    let db_count = repository.count_users().await;
    assert!(db_count == 1);

    let db_item = repository.get_user(user).await;
    assert!(db_item.email() == user);
    assert!(db_item.password() != "test"); // verify password is not saved in plaintext
}

#[rstest]
#[tokio::test]
async fn test_register_user_invalid_email(
    #[values(DatabaseType::MongoDb)] db_type: DatabaseType,
    #[values(true, false)] is_admin: bool,
) {
    let repository = repository::new(&db_type);

    let request = format!(
        r#"{{
        "email": "user",
        "password": "test",
        "admin": {}
    }}"#,
        is_admin
    );

    let response = app::new(&db_type)
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
    assert_eq!(body, json!({"error": "invalid request"}).to_string());

    let db_count = repository.count_users().await;
    assert!(db_count == 0);
}

#[rstest]
#[case(true, "admin@test.com")]
#[case(false, "user@test.com")]
#[tokio::test]
async fn test_register_user_already_registered(
    #[values(DatabaseType::MongoDb)] db_type: DatabaseType,
    #[case] is_admin: bool,
    #[case] user: &str,
) {
    let repository = repository::new(&db_type);

    repository.add_user(user, "test").await;

    let request = format!(
        r#"{{
        "email": "{}",
        "password": "test",
        "admin": {}
    }}"#,
        user, is_admin
    );

    let response = app::new(&db_type)
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
    assert_eq!(
        body,
        json!({"error": "user already registered"}).to_string()
    );

    let db_count = repository.count_users().await;
    assert!(db_count == 1);
}

#[rstest]
#[tokio::test]
async fn test_login_user(#[values(DatabaseType::MongoDb)] db_type: DatabaseType) {
    let repository = repository::new(&db_type);

    let password_hash = Argon2::default()
        .hash_password(b"test", &SaltString::generate(&mut OsRng))
        .unwrap()
        .to_string();
    repository.add_user("user@test.com", &password_hash).await;

    let request = r#"{
        "email": "user@test.com",
        "password": "test"
    }"#;

    let response = app::new(&db_type)
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

#[rstest]
#[tokio::test]
async fn test_login_user_invalid_email(#[values(DatabaseType::MongoDb)] db_type: DatabaseType) {
    repository::new(&db_type);

    let request = r#"{
        "email": "user",
        "password": "test"
    }"#;

    let response = app::new(&db_type)
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
    assert_eq!(body, json!({"error": "invalid request"}).to_string());
}

#[rstest]
#[tokio::test]
async fn test_login_user_not_found(#[values(DatabaseType::MongoDb)] db_type: DatabaseType) {
    let repository = repository::new(&db_type);

    let password_hash = Argon2::default()
        .hash_password(b"test", &SaltString::generate(&mut OsRng))
        .unwrap()
        .to_string();
    repository.add_user("user@test.com", &password_hash).await;

    let request = r#"{
        "email": "user2@test.com",
        "password": "test"
    }"#;

    let response = app::new(&db_type)
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

#[rstest]
#[tokio::test]
async fn test_login_user_incorrect_password(
    #[values(DatabaseType::MongoDb)] db_type: DatabaseType,
) {
    let repository = repository::new(&db_type);

    let password_hash = Argon2::default()
        .hash_password(b"test", &SaltString::generate(&mut OsRng))
        .unwrap()
        .to_string();
    repository.add_user("user@test.com", &password_hash).await;

    let request = r#"{
        "email": "user@test.com",
        "password": "incorrect"
    }"#;

    let response = app::new(&db_type)
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

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body = std::str::from_utf8(&body).unwrap();
    assert_eq!(
        body,
        json!({"error": "incorrect password for user"}).to_string()
    );
}
