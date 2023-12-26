use std::sync::Arc;
use std::{error, fmt};

use axum::Router;

use crate::{
    controller, repository,
    repository::{DynLinks as DynLinksRepository, DynUsers as DynUsersRepository},
    service::{self, DynLinks as DynLinksService, DynUsers as DynUsersService},
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

    let state = State::new(links_service, users_service, links_repo, users_repo);
    Router::new()
        .merge(controller::routes::links::routes(state.clone()))
        .merge(controller::routes::users::routes(state.clone()))
        .with_state(state)
}

#[derive(Clone)]
pub struct State {
    links_service: DynLinksService,
    users_service: DynUsersService,
    links_repo: DynLinksRepository,
    users_repo: DynUsersRepository,
}

#[allow(clippy::must_use_candidate)]
impl State {
    pub fn new(
        links_service: DynLinksService,
        users_service: DynUsersService,
        links_repo: DynLinksRepository,
        users_repo: DynUsersRepository,
    ) -> Self {
        Self {
            links_service,
            users_service,
            links_repo,
            users_repo,
        }
    }

    pub fn links_service(&self) -> &DynLinksService {
        &self.links_service
    }

    pub fn users_service(&self) -> &DynUsersService {
        &self.users_service
    }

    pub fn links_repo(&self) -> &DynLinksRepository {
        &self.links_repo
    }

    pub fn users_repo(&self) -> &DynUsersRepository {
        &self.users_repo
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    ServerError(String),
    DatabaseError(String),
    LinkNotFound(String),
    UserAlreadyExists(String),
    UserNotFound(String),
    IncorrectPassword(String),
    AuthorizationError(String),
    ValidationError(String),

    #[cfg(test)]
    TestError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ServerError(_) => write!(f, "server error"),
            Self::DatabaseError(_) => write!(f, "database error"),
            Self::LinkNotFound(_) => write!(f, "link item not found"),
            Self::UserAlreadyExists(_) => write!(f, "user already registered"),
            Self::UserNotFound(_) => write!(f, "user not found"),
            Self::IncorrectPassword(_) => write!(f, "incorrect password for user"),
            Self::AuthorizationError(_) => write!(f, "invalid authorization token"),
            Self::ValidationError(_) => write!(f, "invalid request"),

            #[cfg(test)]
            Self::TestError => write!(f, "test error"),
        }
    }
}

impl error::Error for Error {}
