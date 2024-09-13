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

    // endregion: Public APIs

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

    // endregion: Generic APIs

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

    // endregion: Helpers
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
    #[builder(setter(skip))]
    /// Whether to allow external API calls
    allow_external_api_call: bool,
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
        let allow_external_api_call = true;
        let api_version = 1;
        let domain = "https://opendart.fss.or.kr".to_string();

        Self {
            allow_external_api_call,
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
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    #[tokio::test]
    async fn test_get_list_default() -> anyhow::Result<()> {
        let test_name = get_test_name();
        let TestContext {
            mut api,
            update_golden_files,
            mock_server,
        } = TestContext::new().await;

        let golden_file_path = format!("tests/resources/{}.json", test_name);
        let golden_file_exists = std::path::Path::new(&golden_file_path).exists();

        // region: Flags
        let allow_external_api_call = api.config.allow_external_api_call;
        let local_response = !allow_external_api_call || golden_file_exists;
        let update_golden_files = allow_external_api_call && update_golden_files;
        // endregion: Flags

        // region: Set up the appropriate domain and mock if needed
        if local_response {
            let body = std::fs::read_to_string(&golden_file_path)
                .context("Failed to read response body from file")?;
            let body: OpenDartResponseBody<List> =
                serde_json::from_str(&body).context("Failed to deserialize response body")?;

            let response = ResponseTemplate::new(200).set_body_json(body);

            Mock::given(method("GET"))
                .and(path("/api/list.json"))
                .respond_with(response)
                .mount(&mock_server)
                .await;

            api.set_domain(&mock_server.uri());
        } else {
            api.set_domain("https://opendart.fss.or.kr");
        }
        // endregion: Set up the appropriate domain and mock if needed

        // region: Perform API call
        let response = api.get_list(Default::default()).await;
        let response = response.context("Response should be successful")?;
        // endregion: Perform API call

        // region: Assert response
        assert!(response.status().is_success());
        let response_body = response.body();
        // endregion: Assert response

        // region: Save response body
        if update_golden_files {
            save_response_body(response_body, &golden_file_path)
                .await
                .context("Failed to save response body")?;
        }
        // endregion: Save response body

        Ok(())
    }
}
