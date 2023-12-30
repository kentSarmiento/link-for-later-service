use axum::async_trait;

use crate::{
    entity::LinkItem,
    service::Analysis as AnalysisService,
    types::{AppError, Result},
};

const ANALYSIS_SERVICE_URL: &str = "ANALYSIS_SERVICE_URL";

pub struct ServiceProvider {
    http_client: reqwest::Client,
}

#[async_trait]
impl AnalysisService for ServiceProvider {
    async fn analyze(&self, link_item: &LinkItem) -> Result<()> {
        match std::env::var(ANALYSIS_SERVICE_URL) {
            Ok(url) => {
                self.http_client
                    .post(url)
                    .json(&link_item)
                    .send()
                    .await
                    .map_err(|e| AppError::Server(format!("client.post() {e:?}")))?;
            }
            Err(_) => {
                tracing::warn!("Analysis Service URL is not set");
            }
        }
        Ok(())
    }
}

impl Default for ServiceProvider {
    fn default() -> Self {
        Self {
            http_client: reqwest::Client::new(),
        }
    }
}
