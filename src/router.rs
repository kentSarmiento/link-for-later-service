use std::sync::Arc;

use axum::Router;

use crate::controller::links;
use crate::repository::{DynLinksRepo, SampleRepo};

pub fn new() -> Router {
    let list_repo = Arc::new(SampleRepo {}) as DynLinksRepo;
    Router::new().merge(links::router()).with_state(list_repo)
}
