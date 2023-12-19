use std::sync::Arc;

use axum::Router;

use crate::{
    controller, repository, service,
    types::{AppState, DynLinksRepo, DynLinksService},
};

pub fn new() -> Router {
    let links_service = Arc::new(service::links::Service {}) as DynLinksService;
    let links_repo = Arc::new(repository::links::Repository {}) as DynLinksRepo;

    let state = AppState::new(links_service, links_repo);
    Router::new()
        .merge(controller::links::routes(state.clone()))
        .with_state(state)
}
