use std::sync::Arc;

use axum::Router;

use crate::controller::links;
use crate::repository::{sample, DynLinksRepo};

pub fn new() -> Router {
    let list_repo = Arc::new(sample::Repo {}) as DynLinksRepo;
    Router::new().merge(links::router()).with_state(list_repo)
}
