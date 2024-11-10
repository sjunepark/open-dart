use crate::client::{OpenDartApi, OpenDartConfigBuilder};
use goldrust::{goldrust, Goldrust, ResponseSource};
use serde::de::DeserializeOwned;
use serde::Serialize;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

macro_rules! test_context {
    ($goldrust_file_extension:expr) => {
        $crate::test_utils::TestContext::new(
            $goldrust_file_extension,
            $crate::test_utils::function_id!(),
        )
    };
}

pub(crate) use test_context;

#[derive(Debug)]
pub(crate) struct TestContext {
    pub(crate) function_id: String,
    pub(crate) api: OpenDartApi,
    pub(crate) mock_server: wiremock::MockServer,
    pub(crate) goldrust: Goldrust,
}

impl TestContext {
    pub async fn new(goldrust_file_extension: &str, function_id: &str) -> Self {
        let goldrust = goldrust!(goldrust_file_extension, function_id);

        // region: Mock server setup
        let mock_server = wiremock::MockServer::start().await;
        // endregion

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
        let api = OpenDartApi::with_config(config);
        // endregion

        Self {
            function_id: function_id.to_string(),
            api,
            mock_server,
            goldrust,
        }
    }

    #[tracing::instrument(skip(self))]
    pub async fn arrange_test_endpoint_json<R>(&mut self, api_path: &str)
    where
        R: Serialize + DeserializeOwned + std::fmt::Debug,
    {
        let response_source = &self.goldrust.response_source;
        let golden_file_path = &self.goldrust.golden_file_path;

        match response_source {
            ResponseSource::Local => {
                tracing::debug!(
                    ?response_source,
                    ?golden_file_path,
                    "Getting response body from file"
                );
                let golden_file_str = std::fs::read_to_string(golden_file_path)
                    .expect("Failed to read response body from file");
                let golden_file_body: R = serde_json::from_str(&golden_file_str)
                    .inspect_err(|e| {
                        tracing::error!(?e, ?golden_file_str, "Failed to deserialize response body")
                    })
                    .unwrap();

                let response = ResponseTemplate::new(200).set_body_json(&golden_file_body);

                Mock::given(method("GET"))
                    .and(path(api_path))
                    .respond_with({
                        tracing::debug!(?golden_file_body, "Responding from mock server");
                        response
                    })
                    .mount(&self.mock_server)
                    .await;
            }
            ResponseSource::External => {
                tracing::debug!(?response_source, "Getting response body from external API");
            }
        }
    }

    #[tracing::instrument(skip(self))]
    pub async fn arrange_test_endpoint_zip(&mut self, api_path: &str) {
        let response_source = &self.goldrust.response_source;
        let golden_file_path = &self.goldrust.golden_file_path;

        match response_source {
            ResponseSource::Local => {
                tracing::debug!(
                    ?response_source,
                    ?golden_file_path,
                    "Getting response body from file"
                );
                let golden_file_bytes = std::fs::read(golden_file_path)
                    .expect("Failed to read response body from file");

                let response = ResponseTemplate::new(200)
                    .set_body_bytes(golden_file_bytes) // Remove & to pass ownership
                    .insert_header("content-type", "application/zip");

                tracing::debug!(?response, "Responding from mock server");

                Mock::given(method("GET"))
                    .and(path(api_path))
                    .respond_with(response) // Simplified this line since we don't need the debug block
                    .mount(&self.mock_server)
                    .await;
            }
            ResponseSource::External => {
                tracing::debug!(?response_source, "Getting response body from external API");
            }
        }
    }
}
