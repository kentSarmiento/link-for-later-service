use axum::async_trait;

use crate::{
    entity::LinkItem,
    service::Analysis as AnalysisService,
    types::{AppError, Result},
};

const ANALYSIS_SERVICE_URL: &str = "ANALYSIS_SERVICE_URL";

pub struct ServiceProvider {
    http_client: reqwest::Client,
    analysis_service_url: String,
}

#[async_trait]
impl AnalysisService for ServiceProvider {
    async fn analyze(&self, link_item: &LinkItem) -> Result<()> {
        if self.analysis_service_url.is_empty() {
            tracing::warn!("Analysis Service URL is not set");
        } else {
            self.http_client
                .post(self.analysis_service_url.clone())
                .json(&link_item)
                .send()
                .await
                .map_err(|e| AppError::Server(format!("client.post() {e:?}")))?;
        }
        Ok(())
    }
}

impl Default for ServiceProvider {
    fn default() -> Self {
        Self {
            http_client: reqwest::Client::new(),
            analysis_service_url: std::env::var(ANALYSIS_SERVICE_URL)
                .map_or_else(|_| String::default(), |url| url),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::entity::LinkItemBuilder;

    use super::*;

    #[allow(clippy::significant_drop_tightening)]
    #[tokio::test]
    async fn test_analyze_link() {
        let item = LinkItemBuilder::new("http://link")
            .id("1")
            .owner("user-id")
            .build();

        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/")
            .with_status(202)
            .match_body(mockito::Matcher::AllOf(vec![
                mockito::Matcher::Regex("http://link".to_owned()),
                mockito::Matcher::Regex("1".to_owned()),
                mockito::Matcher::Regex("user-id".to_owned()),
            ]))
            .create();

        let analysis_service = ServiceProvider {
            http_client: reqwest::Client::new(),
            analysis_service_url: server.url(),
        };

        let response = analysis_service.analyze(&item).await;

        mock.assert();
        assert!(response.is_ok());
    }
}
