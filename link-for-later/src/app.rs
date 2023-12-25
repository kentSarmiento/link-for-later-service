use std::sync::Arc;

use axum::Router;

use crate::{
    controller, repository,
    repository::{DynLinks as DynLinksRepository, DynUsers as DynUsersRepository},
    service::{self, DynLinks as DynLinksService, DynUsers as DynUsersService},
    state::AppState,
    types::Database,
};

pub fn new(db: Database) -> Router {
    let links_service = Arc::new(service::links::ServiceProvider {}) as DynLinksService;
    let users_service = Arc::new(service::users::ServiceProvider {}) as DynUsersService;
    let (links_repo, users_repo) = match db {
        Database::MongoDb(db) => (
            Arc::new(repository::mongodb::LinksRepositoryProvider::new(&db)) as DynLinksRepository,
            Arc::new(repository::mongodb::UsersRepositoryProvider::new(&db)) as DynUsersRepository,
        ),
        Database::InMemory => (
            Arc::new(repository::inmemory::LinksRepositoryProvider::default())
                as DynLinksRepository,
            Arc::new(repository::inmemory::UsersRepositoryProvider::default())
                as DynUsersRepository,
        ),
    };

    let state = AppState::new(links_service, users_service, links_repo, users_repo);
    Router::new()
        .merge(controller::links::routes(state.clone()))
        .merge(controller::users::routes(state.clone()))
        .with_state(state)
}
