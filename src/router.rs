use std::sync::Arc;

use axum::Router;

use crate::{
    controller, repository, service,
    types::{repository::DynLinks as DynLinksRepo, service::DynLinks as DynLinksService, state},
};

pub fn new() -> Router {
    let links_repo = Arc::new(repository::links::Repository {}) as DynLinksRepo;
    let links_service = Arc::new(service::links::Service {}) as DynLinksService;

    let state = state::Router::new(links_repo, links_service);
    Router::new()
        .merge(controller::links::router())
        .with_state(state)
}
