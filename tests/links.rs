use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::Value;
use tower::ServiceExt;

// Verifies GET /v1/links request
//
// GIVEN    an empty set of link items in the Server
// WHEN     GET /v1/links request is sent to the Server
// THEN     the response is 200 OK and empty set of link items are returned
#[tokio::test]
async fn test_get_links() {
    let handler = link_for_later::router::new();
    let response = handler
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/v1/links")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], b"[]");
}

// Verifies POST /v1/links request
//
// GIVEN    an empty set of link items in the Server
// AND      a request to post a link
// WHEN     POST /v1/links request is sent to the Server
// THEN     the response is 201 CREATED and created link item is returned
#[tokio::test]
async fn test_post_links() {
    let request = r#"{
        "url": "http://link"
    }"#;

    let handler = link_for_later::router::new();
    let response = handler
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/links")
                .header("Content-Type", "application/json")
                .body(Body::from(request))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body = std::str::from_utf8(&body).unwrap();
    let body: Value = serde_json::from_str(body).unwrap();

    assert!(body["id"] != "");
    assert!(body["owner"] == "");
    assert!(body["url"] == "http://link");
    assert!(body["title"] == "");
    assert!(body["description"] == "");
    assert!(body["created_at"] != "");
    assert!(body["updated_at"] == "");
}
