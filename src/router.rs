use std::sync::Arc;

use axum::Router;

use crate::{
    controller, repository, service,
    types::{
        repository::DynLinks as DynLinksRepo, service::DynLinks as DynLinksService,
        state::Router as RouterState,
    },
};

pub fn new() -> Router {
    let links_service = Arc::new(service::links::Service {}) as DynLinksService;
    let links_repo = Arc::new(repository::links::Repository {}) as DynLinksRepo;

    let state = RouterState::new(links_service, links_repo);
    Router::new()
        .merge(controller::links::router())
        .with_state(state)
}
