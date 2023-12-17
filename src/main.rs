use axum::{extract::Path, response::Json, routing::get, Router};
use lambda_http::{run, Error};
use serde_json::{json, Value};

async fn get_links() -> Json<Value> {
    Json(json!({ "msg": format!("I am GET /link") }))
}

async fn get_link(Path(id): Path<String>) -> Json<Value> {
    Json(json!({ "msg": format!("I am GET /link/:id, id={id}") }))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let app = Router::new()
        .route("/link", get(get_links))
        .route("/link/:id", get(get_link));

    run(app).await
}
