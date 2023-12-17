use axum::{routing::get, Router};

use crate::controller::link;

pub fn new() -> Router {
    Router::new()
        .route("/link", get(link::list).post(link::post))
        .route(
            "/link/:id",
            get(link::get).put(link::put).delete(link::delete),
        )
}
