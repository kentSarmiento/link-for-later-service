#![allow(dead_code)]

use std::collections::HashMap;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use rstest::rstest;
use serde_json::json;
use tower::ServiceExt;

use link_for_later_types::entity::LinkItem;

use crate::repository::DatabaseType;

mod app;
mod auth;
mod repository;

#[rstest]
#[case(true, "admin@test.com")]
#[case(false, "user@test.com")]
#[tokio::test]
async fn test_get_links_empty(
    #[values(DatabaseType::InMemory, DatabaseType::MongoDb)] db_type: DatabaseType,
    #[case] is_admin: bool,
    #[case] user: &str,
) {
    repository::new(&db_type);

    let token = auth::generate_token(user, is_admin);

    let response = app::new(&db_type)
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

#[rstest]
#[case(true, "admin@test.com")]
#[case(false, "user@test.com")]
#[tokio::test]
async fn test_get_links_non_empty(
    #[values(DatabaseType::MongoDb)] db_type: DatabaseType,
    #[case] is_admin: bool,
    #[case] user: &str,
) {
    let repository = repository::new(&db_type);

    let id = repository.add_link("user@test.com", "http://test").await;
    let token = auth::generate_token(user, is_admin);

    let response = app::new(&db_type)
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
    assert!(body[0].id() == id);
    assert!(body[0].owner() == "user@test.com");
    assert!(body[0].url() == "http://test");
}

#[rstest]
#[case(true, "admin@test.com")]
#[case(false, "user@test.com")]
#[tokio::test]
async fn test_get_links_multiple_entries(
    #[values(DatabaseType::MongoDb)] db_type: DatabaseType,
    #[case] is_admin: bool,
    #[case] user: &str,
) {
    let repository = repository::new(&db_type);

    let mut ids: HashMap<String, (String, String)> = HashMap::new();

    let id = repository.add_link("user@test.com", "http://test1").await;
    ids.insert(id, ("user@test.com".into(), "http://test1".into()));

    let second_entry_owner = if is_admin {
        "another-user@test.com"
    } else {
        user
    };
    let id = repository
        .add_link(second_entry_owner, "http://test2")
        .await;
    ids.insert(id, (second_entry_owner.into(), "http://test2".into()));

    let token = auth::generate_token(user, is_admin);

    let response = app::new(&db_type)
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

    assert!(body.len() == ids.len());
    for item in body {
        assert!(ids.contains_key(item.id()));
        assert!(ids.get(item.id()).unwrap().0 == item.owner());
        assert!(ids.get(item.id()).unwrap().1 == item.url());
    }
}

#[rstest]
#[case(true, "admin@test.com")]
#[case(false, "user@test.com")]
#[tokio::test]
async fn test_get_link_item_found(
    #[values(DatabaseType::MongoDb)] db_type: DatabaseType,
    #[case] is_admin: bool,
    #[case] user: &str,
) {
    let repository = repository::new(&db_type);

    let id = repository.add_link("user@test.com", "http://test").await;
    let token = auth::generate_token(user, is_admin);

    let response = app::new(&db_type)
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
    assert!(body.id() == id);
    assert!(body.owner() == "user@test.com");
    assert!(body.url() == "http://test");
}

#[rstest]
#[case(true, "admin@test.com")]
#[case(false, "user@test.com")]
#[tokio::test]
async fn test_get_link_item_not_found(
    #[values(DatabaseType::MongoDb)] db_type: DatabaseType,
    #[case] is_admin: bool,
    #[case] user: &str,
) {
    let repository = repository::new(&db_type);

    repository.add_link("user@test.com", "http://test").await;
    let token = auth::generate_token(user, is_admin);

    let response = app::new(&db_type)
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

#[rstest]
#[tokio::test]
async fn test_post_link(
    #[values(DatabaseType::MongoDb)] db_type: DatabaseType,
    #[values(true, false)] is_admin: bool,
) {
    let repository = repository::new(&db_type);

    let token = auth::generate_token("user@test.com", is_admin);
    let request = r#"{
        "url": "http://test"
    }"#;

    let response = app::new(&db_type)
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

    assert!(body.owner() == "user@test.com");
    assert!(body.url() == "http://test");

    let db_count = repository.count_links().await;
    assert!(db_count == 1);

    let db_item = repository.get_link(body.id()).await;
    assert!(db_item.owner() == "user@test.com");
    assert!(db_item.url() == "http://test");
}

#[rstest]
#[tokio::test]
async fn test_post_link_invalid_url(
    #[values(DatabaseType::MongoDb)] db_type: DatabaseType,
    #[values(true, false)] is_admin: bool,
) {
    let repository = repository::new(&db_type);

    let token = auth::generate_token("user@test.com", is_admin);
    let request = r#"{
        "url": "invalid"
    }"#;

    let response = app::new(&db_type)
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
    assert_eq!(body, json!({"error": "invalid request"}).to_string());

    let db_count = repository.count_links().await;
    assert!(db_count == 0);
}

#[rstest]
#[case(true, "admin@test.com")]
#[case(false, "user@test.com")]
#[tokio::test]
async fn test_put_link(
    #[values(DatabaseType::MongoDb)] db_type: DatabaseType,
    #[case] is_admin: bool,
    #[case] user: &str,
) {
    let repository = repository::new(&db_type);

    let id = repository.add_link("user@test.com", "http://test").await;
    let token = auth::generate_token(user, is_admin);

    let request = r#"{
        "url": "http://update"
    }"#;

    let response = app::new(&db_type)
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

    assert!(body.id() == id);
    assert!(body.owner() == "user@test.com");
    assert!(body.url() == "http://update");

    let db_count = repository.count_links().await;
    assert!(db_count == 1);

    let db_item = repository.get_link(&id).await;
    assert!(db_item.id() == id);
    assert!(db_item.owner() == "user@test.com");
    assert!(db_item.url() == "http://update");
}

#[rstest]
#[case(true, "admin@test.com")]
#[case(false, "user@test.com")]
#[tokio::test]
async fn test_put_link_invalid_url(
    #[values(DatabaseType::MongoDb)] db_type: DatabaseType,
    #[case] is_admin: bool,
    #[case] user: &str,
) {
    let repository = repository::new(&db_type);

    let id = repository.add_link("user@test.com", "http://test").await;
    let token = auth::generate_token(user, is_admin);

    let request = r#"{
        "url": "invalid"
    }"#;

    let response = app::new(&db_type)
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
    assert_eq!(body, json!({"error": "invalid request"}).to_string());

    let db_count = repository.count_links().await;
    assert!(db_count == 1);
}

#[rstest]
#[case(true, "admin@test.com")]
#[case(false, "user@test.com")]
#[tokio::test]
async fn test_put_link_item_not_found(
    #[values(DatabaseType::MongoDb)] db_type: DatabaseType,
    #[case] is_admin: bool,
    #[case] user: &str,
) {
    let repository = repository::new(&db_type);

    let id = repository.add_link("user@test.com", "http://test").await;
    let token = auth::generate_token(user, is_admin);

    let request = r#"{
        "url": "http://update"
    }"#;

    let response = app::new(&db_type)
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

    let db_count = repository.count_links().await;
    assert!(db_count == 1);

    let db_item = repository.get_link(&id).await;
    assert!(db_item.id() == id);
    assert!(db_item.owner() == "user@test.com");
    assert!(db_item.url() == "http://test"); // not updated
}

#[rstest]
#[case(true, "admin@test.com")]
#[case(false, "user@test.com")]
#[tokio::test]
async fn test_delete_link(
    #[values(DatabaseType::MongoDb)] db_type: DatabaseType,
    #[case] is_admin: bool,
    #[case] user: &str,
) {
    let repository = repository::new(&db_type);

    let id = repository.add_link("user@test.com", "http://test").await;
    let token = auth::generate_token(user, is_admin);

    let response = app::new(&db_type)
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

    let db_count = repository.count_links().await;
    assert!(db_count == 0);
}

#[rstest]
#[case(true, "admin@test.com")]
#[case(false, "user@test.com")]
#[tokio::test]
async fn test_delete_link_item_not_found(
    #[values(DatabaseType::MongoDb)] db_type: DatabaseType,
    #[case] is_admin: bool,
    #[case] user: &str,
) {
    let repository = repository::new(&db_type);

    let id = repository.add_link("user@test.com", "http://test").await;
    let token = auth::generate_token(user, is_admin);

    let response = app::new(&db_type)
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

    let db_count = repository.count_links().await;
    assert!(db_count == 1);

    let db_item = repository.get_link(&id).await;
    assert!(db_item.id() == id);
    assert!(db_item.owner() == "user@test.com");
    assert!(db_item.url() == "http://test");
}

#[rstest]
#[tokio::test]
async fn test_unauthorized_access_to_links_no_token(
    #[values(DatabaseType::MongoDb)] db_type: DatabaseType,
) {
    repository::new(&db_type);

    let response = app::new(&db_type)
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

#[rstest]
#[tokio::test]
async fn test_unauthorized_access_to_links_invalid_token(
    #[values(DatabaseType::MongoDb)] db_type: DatabaseType,
) {
    repository::new(&db_type);

    let response = app::new(&db_type)
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
