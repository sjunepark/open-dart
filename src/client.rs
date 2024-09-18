use crate::endpoints::{List, ListRequestParams, OpenDartResponse, OpenDartResponseBody};
use crate::error::OpenDartError;
use derive_builder::Builder;
use reqwest;
use reqwest::IntoUrl;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Display;

#[derive(Debug)]
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
        R: Serialize + DeserializeOwned + std::fmt::Debug,
    {
        let request = self.client.get(url).query(&params).build()?;
        let response = self.client.execute(request).await?;
        tracing::trace!(response = ?response, "Got response");

        let headers = response.headers().clone();
        let status = response.status();

        let response_body = response.json::<OpenDartResponseBody<R>>().await?;

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

#[derive(Builder, Clone, Debug)]
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
    use crate::endpoints::{List, ListRequestParamsBuilder};
    use crate::test_utils::MockDefault;
    use crate::types::{BgnDe, CorpCode};
    use crate::TestContext;
    use anyhow::Context;

    #[tokio::test]
    #[tracing::instrument]
    async fn get_list_default() -> anyhow::Result<()> {
        let mut test_context = TestContext::new().await;

        test_context
            .arrange_test_endpoint::<List>("/api/list.json")
            .await?;

        // region: Action
        let params = ListRequestParamsBuilder::default()
            .corp_code(CorpCode::mock_default())
            .bgn_de(BgnDe::mock_default())
            .build()?;
        tracing::debug!(?params, "Request parameters");

        let response = test_context
            .api
            .get_list(params)
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
        test_context.goldrust.save(response.body)?;
        // endregion

        Ok(())
    }
}
