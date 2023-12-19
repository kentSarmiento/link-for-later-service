use std::sync::Arc;

use axum::Router;
use mongodb::Database;

use crate::{
    controller, repository, service,
    types::{AppState, DynLinksRepo, DynLinksService},
};

pub fn new(db: Option<Database>) -> Router {
    let links_service = Arc::new(service::links::Service {}) as DynLinksService;
    let links_repo = db.map_or_else(
        || Arc::new(repository::links::MongoDbRepository::default()) as DynLinksRepo,
        |db| Arc::new(repository::links::MongoDbRepository::new(&db)) as DynLinksRepo,
    );

    let state = AppState::new(links_service, links_repo);
    Router::new()
        .merge(controller::links::routes(state.clone()))
        .with_state(state)
}
