use std::sync::Arc;

use axum::Router;

use crate::{
    controller, repository, service,
    types::{AppState, DynLinksRepo, DynLinksService, Repository},
};

pub fn new(db: Repository) -> Router {
    let links_service = Arc::new(service::links::Service {}) as DynLinksService;
    let links_repo = match db {
        Repository::MongoDb(db) => {
            Arc::new(repository::mongodb::MongoDbRepository::new(&db)) as DynLinksRepo
        }
        Repository::None => Arc::new(repository::Base::default()) as DynLinksRepo,
    };

    let state = AppState::new(links_service, links_repo);
    Router::new()
        .merge(controller::links::routes(state.clone()))
        .with_state(state)
}
