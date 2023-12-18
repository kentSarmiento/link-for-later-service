use super::{repository::DynLinks as DynLinksRepo, service::DynLinks as DynLinksService};

#[derive(Clone)]
pub struct Router {
    links_service: DynLinksService,
    links_repo: DynLinksRepo,
}

impl Router {
    pub fn new(links_service: DynLinksService, links_repo: DynLinksRepo) -> Self {
        Self {
            links_service,
            links_repo,
        }
    }

    pub fn get_links_service(&self) -> &DynLinksService {
        &self.links_service
    }

    pub fn get_links_repo(&self) -> &DynLinksRepo {
        &self.links_repo
    }
}
