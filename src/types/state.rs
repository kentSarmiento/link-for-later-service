use super::{DynLinksRepo, DynLinksService};

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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::types::{
        repository::MockLinks as MockRepository, service::MockLinks as MockService,
    };

    use super::*;

    #[test]
    fn test_router_state() {
        let mock_links_service = Arc::new(MockService::new()) as DynLinksService;
        let mock_links_repo = Arc::new(MockRepository::new()) as DynLinksRepo;

        let router_state = Router::new(mock_links_service.clone(), mock_links_repo.clone());

        assert!(Arc::ptr_eq(
            &mock_links_service,
            router_state.get_links_service()
        ));
        assert!(Arc::ptr_eq(&mock_links_repo, router_state.get_links_repo()));
    }
}
