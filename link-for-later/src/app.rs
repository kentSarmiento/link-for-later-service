use std::sync::Arc;

use axum::Router;

use crate::{
    controller, repository,
    repository::{DynLinks as DynLinksRepository, DynUsers as DynUsersRepository},
    service::{self, DynLinks as DynLinksService},
    state::AppState,
    types::Database,
};

pub fn new(db: Database) -> Router {
    let links_service = Arc::new(service::links::ServiceProvider {}) as DynLinksService;
    let (links_repo, users_repo) = match db {
        Database::MongoDb(db) => (
            Arc::new(repository::mongodb::LinksDb::new(&db)) as DynLinksRepository,
            Arc::new(repository::mongodb::UsersDb::new(&db)) as DynUsersRepository,
        ),
        Database::None => (
            Arc::new(repository::BareDb::default()) as DynLinksRepository,
            Arc::new(repository::BareDb::default()) as DynUsersRepository,
        ),
    };

    let state = AppState::new(links_service, links_repo, users_repo);
    Router::new()
        .merge(controller::links::routes(state.clone()))
        .merge(controller::users::routes(state.clone()))
        .with_state(state)
}
