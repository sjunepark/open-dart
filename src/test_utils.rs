#![cfg(test)]

use crate::client::{OpenDartApi, OpenDartConfigBuilder};
use crate::endpoints::OpenDartResponseBody;
use anyhow::Context;
use goldrust::{Goldrust, ResponseSource};
use serde::de::DeserializeOwned;
use serde::Serialize;
use tracing::metadata::LevelFilter;
use tracing_log::AsLog;
use tracing_subscriber::EnvFilter;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[derive(Debug)]
pub struct TestContext {
    pub api: OpenDartApi,
    pub mock_server: wiremock::MockServer,
    goldrust: Goldrust,
}

impl TestContext {
    pub async fn new() -> Self {
        // region: Tracing setup
        let subscriber = tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .pretty()
            .finish();

        let _ = tracing::subscriber::set_global_default(subscriber);

        let current_level = LevelFilter::current();

        let _ = tracing_log::LogTracer::builder()
            // Note that we must call this *after* setting the global default subscriber
            // so that we get its max level hint.
            .with_max_level(current_level.as_log())
            .init();
        // endregion

        // region: Mock server setup
        let mock_server = wiremock::MockServer::start().await;
        // endregion

        let goldrust = Goldrust::default();

        // region: Goldrust setup
        let domain = match goldrust.response_source {
            ResponseSource::Local => mock_server.uri(),
            ResponseSource::External => "https://opendart.fss.or.kr".to_string(),
        };

        // endregion

        // region: Initialize OpenDartApi
        let config = OpenDartConfigBuilder::default()
            .domain(domain)
            .build()
            .expect("Failed to build OpenDartConfig");
        let api = OpenDartApi::new(config);
        // endregion

        Self {
            api,
            mock_server,
            goldrust,
        }
    }

    #[tracing::instrument(skip(self))]
    pub async fn test_endpoint_default<R>(&mut self, api_path: &str) -> anyhow::Result<()>
    where
        R: Serialize + DeserializeOwned,
    {
        let response_source = &self.goldrust.response_source;
        let golden_file_path = &self.goldrust.golden_file_path;

        // region: Arrange
        match response_source {
            ResponseSource::Local => {
                tracing::debug!(
                    ?response_source,
                    ?golden_file_path,
                    "Getting response body from file"
                );
                let golden_file_str = std::fs::read_to_string(golden_file_path)
                    .context("Failed to read response body from file")?;
                let golden_file_body: OpenDartResponseBody<R> =
                    serde_json::from_str(&golden_file_str)
                        .context("Failed to deserialize response body")?;

                let response = ResponseTemplate::new(200).set_body_json(&golden_file_body);

                Mock::given(method("GET"))
                    .and(path(api_path.to_string()))
                    .respond_with(response)
                    .mount(&self.mock_server)
                    .await;
            }
            ResponseSource::External => {
                tracing::debug!(?response_source, "Getting response body from external API");
            }
        }
        // endregion

        // region: Action
        let response = self
            .api
            .get_list(Default::default())
            .await
            .context("get_list should succeed")?;
        // endregion

        // region: Assert
        assert!(
            response.status().is_success(),
            "Response didn't return a status code of 2xx"
        );
        // endregion

        // region: Save response body
        self.goldrust.save(response.body)?;
        // endregion

        Ok(())
    }
}

pub trait MockDefault: Sized {
    fn mock_default() -> Self;
}
