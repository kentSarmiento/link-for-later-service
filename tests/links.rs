use std::error::Error;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use tower::ServiceExt;

// Verifies GET /v1/links request
//
// GIVEN empty set of link items in the Server
// WHEN GET /v1/links request is sent to the Server
// THEN the response is 200 OK and empty set of link items are returned
#[tokio::test]
async fn test_get_links_empty() -> Result<(), Box<dyn Error>> {
    let handler = link_for_later::router::new();
    let response = handler
        .oneshot(
            Request::builder()
                .uri("/v1/links")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], b"[]");
    Ok(())
}
