use crate::repository::{DynLinks as DynLinksRepository, DynUsers as DynUsersRepository};
use crate::service::DynLinks as DynLinksService;

#[derive(Clone)]
pub struct App {
    links_service: DynLinksService,
    links_repo: DynLinksRepository,
    users_repo: DynUsersRepository,
}

impl App {
    pub fn new(
        links_service: DynLinksService,
        links_repo: DynLinksRepository,
        users_repo: DynUsersRepository,
    ) -> Self {
        Self {
            links_service,
            links_repo,
            users_repo,
        }
    }

    pub fn links_service(&self) -> &DynLinksService {
        &self.links_service
    }

    pub fn links_repo(&self) -> &DynLinksRepository {
        &self.links_repo
    }

    pub fn users_repo(&self) -> &DynUsersRepository {
        &self.users_repo
    }
}
