use crate::endpoints::{List, ListRequestParams, OpenDartResponse, OpenDartResponseBody};
use crate::error::{map_deserialization_error, OpenDartError};
use derive_builder::Builder;
use reqwest;
use reqwest::IntoUrl;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Display;

pub struct OpenDartApi {
    client: reqwest::Client,
    config: OpenDartConfig,
}

impl OpenDartApi {
    fn url(&self, path: &str) -> String {
        if !path.starts_with("/") {
            panic!("Path must start with a slash");
        }
        format!("{}{}", self.config.domain, path)
    }

    // region: Public APIs
    pub fn new(config: OpenDartConfig) -> Self {
        if config.api_version != 1 {
            panic!("The only supported API version is 1");
        }

        Self {
            client: reqwest::Client::builder()
                .default_headers(Self::default_headers())
                .build()
                .expect("Failed to build reqwest client"),
            config,
        }
    }

    pub fn set_domain(&mut self, domain: &str) {
        self.config.domain = domain.to_string();
    }

    pub async fn get_list(
        &self,
        args: ListRequestParams,
    ) -> Result<OpenDartResponse<List>, OpenDartError> {
        self.get(self.url("/api/list.json"), args).await
    }

    // endregion

    // region: Generic APIs
    async fn get<'de, U, P, R>(
        &self,
        url: U,
        params: P,
    ) -> Result<OpenDartResponse<R>, OpenDartError>
    where
        U: Display + IntoUrl,
        P: Serialize,
        R: Serialize + DeserializeOwned,
    {
        let request = self.client.get(url).query(&params).build()?;
        let response = self.client.execute(request).await?;

        let headers = response.headers().clone();
        let status = response.status();

        let bytes = response.bytes().await?;
        let response_body: OpenDartResponseBody<R> =
            serde_json::from_slice(&bytes).map_err(|e| map_deserialization_error(e, &bytes))?;

        let response = OpenDartResponse::new(status, headers, response_body);
        Ok(response)
    }

    // endregion

    // region: Helpers
    fn default_headers() -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_static(
                "Mozilla/4.0 (compatible; MSIE 5.01; Windows NT 5.0)",
            ),
        );
        headers.insert(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static(
                "application/json, application/xml, application/zip",
            ),
        );
        headers
    }
    // endregion
}

impl Default for OpenDartApi {
    /// Create a new `OpenDartApi` instance with the default configuration.
    ///
    /// The default configuration is as below:
    /// - `api_version`: 1
    /// - `domain`: "https://opendart.fss.or.kr"
    fn default() -> Self {
        Self::new(OpenDartConfig::default())
    }
}

#[derive(Builder, Clone)]
#[builder(default)]
pub struct OpenDartConfig {
    /// API version to use
    api_version: u32,
    /// The domain, which will default to 'https://opendart.fss.or.kr'
    /// This field exists to be adjusted in testing environments
    domain: String,
}

impl OpenDartConfig {
    pub fn set_domain(&mut self, domain: &str) {
        self.domain = domain.to_string();
    }
}

impl Default for OpenDartConfig {
    fn default() -> Self {
        let api_version = 1;
        let domain = "https://opendart.fss.or.kr".to_string();

        Self {
            api_version,
            domain,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::endpoints::{List, OpenDartResponseBody};
    use crate::test_utils::{get_test_name, save_response_body};
    use crate::TestContext;
    use anyhow::Context;
    use std::time::SystemTime;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    #[tokio::test]
    async fn test_get_list_default() -> anyhow::Result<()> {
        // region: Arrange
        let test_name = get_test_name();
        let TestContext {
            mut api,
            mock_server,
            allow_external_api_call,
            update_golden_files,
        } = TestContext::new().await;

        let golden_file_path = format!("tests/resources/{}.json", test_name);
        let golden_file_exists = std::path::Path::new(&golden_file_path).exists();
        let mut golden_file_body: Option<OpenDartResponseBody<List>> = None;

        #[derive(Debug)]
        enum ResponseSource {
            Local,
            External,
        }

        let response_source: ResponseSource = match (
            allow_external_api_call,
            update_golden_files,
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
                    .and(path("/api/list.json"))
                    .respond_with(response)
                    .mount(&mock_server)
                    .await;

                api.set_domain(&mock_server.uri());
            }
            ResponseSource::External => {
                tracing::debug!(response_source = ?response_source, file_path = ?golden_file_path, "Getting response body from external API");
                api.set_domain("https://opendart.fss.or.kr");
            }
        }
        // endregion

        // region: Action
        let response = api
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
        if update_golden_files {
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
