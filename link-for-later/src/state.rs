use crate::{
    repository::{DynLinks as DynLinksRepository, DynUsers as DynUsersRepository},
    service::{DynLinks as DynLinksService, DynUsers as DynUsersService},
};

#[allow(clippy::module_name_repetitions)]
#[derive(Clone)]
pub struct AppState {
    links_service: DynLinksService,
    users_service: DynUsersService,
    links_repo: DynLinksRepository,
    users_repo: DynUsersRepository,
}

impl AppState {
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
