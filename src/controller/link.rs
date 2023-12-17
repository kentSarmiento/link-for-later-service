use axum::{extract::Path, response::Json};
use serde_json::{json, Value};

pub async fn list() -> Json<Value> {
    Json(json!({ "msg": format!("GET /link") }))
}

pub async fn post() -> Json<Value> {
    Json(json!({ "msg": format!("POST /link") }))
}

pub async fn get(Path(id): Path<String>) -> Json<Value> {
    Json(json!({ "msg": format!("GET /link/:id, id={id}") }))
}

pub async fn put(Path(id): Path<String>) -> Json<Value> {
    Json(json!({ "msg": format!("PUT /link/:id, id={id}") }))
}

pub async fn delete(Path(id): Path<String>) -> Json<Value> {
    Json(json!({ "msg": format!("DELETE /link/:id, id={id}") }))
}
