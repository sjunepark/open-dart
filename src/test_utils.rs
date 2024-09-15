#![cfg(test)]

use crate::client::{OpenDartApi, OpenDartConfigBuilder};
use crate::endpoints::{List, OpenDartResponseBody};
use anyhow::Context;
use serde::Serialize;
use std::time::SystemTime;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tracing::metadata::LevelFilter;
use tracing_log::AsLog;
use tracing_subscriber::EnvFilter;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[derive(Debug)]
pub struct TestContext {
    pub api: OpenDartApi,
    pub mock_server: wiremock::MockServer,
    pub allow_external_api_call: bool,
    pub update_golden_files: bool,
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

        // region: Set domain for external open dart api calls
        let allow_external_call: bool = std::env::var("ALLOW_EXTERNAL_API_CALL")
            .expect("ALLOW_EXTERNAL_API_CALL must be set")
            .parse()
            .expect(r#"ALLOW_EXTERNAL_API_CALL must be a boolean, e.g., "true" or "false""#);

        let domain = if allow_external_call {
            "https://opendart.fss.or.kr".to_string()
        } else {
            mock_server.uri()
        };
        // endregion

        // region: Initialize OpenDartApi
        let config = OpenDartConfigBuilder::default()
            .domain(domain)
            .build()
            .expect("Failed to build OpenDartConfig");
        let api = OpenDartApi::new(config);
        // endregion

        // region: Update flag setup
        let update_golden_files: bool = std::env::var("UPDATE_GOLDEN_FILES")
            .expect("UPDATE_GOLDEN_FILES must be set")
            .parse()
            .expect("UPDATE_GOLDEN_FILES must be a boolean");

        let allow_external_api_call: bool = std::env::var("ALLOW_EXTERNAL_API_CALL")
            .expect("ALLOW_EXTERNAL_API_CALL must be set")
            .parse()
            .expect("ALLOW_EXTERNAL_API_CALL must be a boolean");
        // endregion

        Self {
            api,
            mock_server,
            allow_external_api_call,
            update_golden_files,
        }
    }

    #[tracing::instrument(skip(self))]
    pub async fn test_endpoint_default(
        &mut self,
        test_name: &str,
        api_path: &str,
    ) -> anyhow::Result<()> {
        // region: Arrange
        let golden_file_path = format!("tests/resources/{}.json", test_name);
        let golden_file_exists = std::path::Path::new(&golden_file_path).exists();
        let mut golden_file_body: Option<OpenDartResponseBody<List>> = None;

        #[derive(Debug)]
        enum ResponseSource {
            Local,
            External,
        }

        let response_source: ResponseSource = match (
            self.allow_external_api_call,
            self.update_golden_files,
            golden_file_exists,
        ) {
            (false, true, _) => {
                panic!("Cannot update golden files without allowing external API calls")
            }
            (false, false, false) => {
                panic!("Cannot test without allowing external API calls when golden files do not exist")
            }
            // Get from local without making external API calls
            (false, false, true) => ResponseSource::Local,
            // Get from external API without updating golden files
            (true, false, false) => ResponseSource::External,
            // Even if external API calls are allowed, respond from local if golden files exist
            (true, false, true) => ResponseSource::Local,
            // Allow external API calls and update golden files
            (true, true, _) => ResponseSource::External,
        };

        match response_source {
            ResponseSource::Local => {
                tracing::debug!(response_source = ?response_source, file_path = ?golden_file_path, "Getting response body from file");
                let golden_file_str = std::fs::read_to_string(&golden_file_path)
                    .context("Failed to read response body from file")?;
                golden_file_body = serde_json::from_str(&golden_file_str)
                    .context("Failed to deserialize response body")?;

                let response = ResponseTemplate::new(200).set_body_json(&golden_file_body);

                Mock::given(method("GET"))
                    .and(path(api_path.to_string()))
                    .respond_with(response)
                    .mount(&self.mock_server)
                    .await;

                self.api.set_domain(&self.mock_server.uri());
            }
            ResponseSource::External => {
                tracing::debug!(response_source = ?response_source, "Getting response body from external API");
                self.api.set_domain("https://opendart.fss.or.kr");
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
        if self.update_golden_files {
            match golden_file_body {
                // When the local file's status is success but the external response body's status is not success,
                // Don't save the external response body to the local file
                Some(body) if body.is_success() && !response.body().is_success() => {
                    tracing::debug!(response_body = ?response.body(), file_path = ?golden_file_path, "External response body is not success, not saving to file");
                }
                _ => {
                    tracing::debug!(response_body = ?response.body(), file_path = ?golden_file_path, "Saving response body to file");
                    save_response_body(response.body(), &golden_file_path)
                        .await
                        .context("Failed to save response body")?;

                    assert!(
                        SystemTime::now()
                            .duration_since(std::fs::metadata(&golden_file_path)?.modified()?)?
                            < std::time::Duration::from_secs(60),
                        "The golden file was not updated within the last minute"
                    );
                }
            }
        }
        // endregion

        Ok(())
    }
}

pub async fn save_response_body<R: Serialize>(
    body: &OpenDartResponseBody<R>,
    path: &str,
) -> anyhow::Result<()> {
    let bytes = serde_json::to_string_pretty(body).expect("Failed to serialize response body");
    let bytes = bytes.as_bytes();
    let mut file = File::create(path).await.expect("Failed to create file");
    file.write_all(bytes).await.map_err(anyhow::Error::from)
}

pub fn get_test_name() -> String {
    std::thread::current()
        .name()
        .expect("Test name must be set")
        .split("::")
        .collect::<Vec<_>>()
        .join("-")
        .to_string()
}

pub trait MockDefault: Sized {
    fn mock_default() -> Self;
}
