use super::{repository::DynLinks as DynLinksRepo, service::DynLinks as DynLinksService};

#[derive(Clone)]
pub struct Router {
    links_repo: DynLinksRepo,
    links_service: DynLinksService,
}

impl Router {
    pub fn new(links_repo: DynLinksRepo, links_service: DynLinksService) -> Self {
        Self {
            links_repo,
            links_service,
        }
    }

    pub fn get_links_repo(&self) -> &DynLinksRepo {
        &self.links_repo
    }

    pub fn get_links_service(&self) -> &DynLinksService {
        &self.links_service
    }
}
