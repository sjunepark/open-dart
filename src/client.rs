use crate::endpoints::{List, ListRequestParams};
use crate::error::OpenDartError;

use crate::endpoints::base::{OpenDartResponse, OpenDartResponseBody};
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
        Self {
            client: reqwest::Client::builder()
                .default_headers(Self::default_headers())
                .build()
                .expect("Failed to build reqwest client"),
            config,
        }
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
    // endregion: Public APIs
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
    /// The domain, which will default to 'https://opendart.fss.or.kr'
    /// This field exists to be adjusted in testing environments
    domain: String,
}

impl Default for OpenDartConfig {
    fn default() -> Self {
        let domain = "https://opendart.fss.or.kr".to_string();

        Self { domain }
    }
}

#[cfg(test)]
mod tests {
    use crate::endpoints::{List, ListRequestParamsBuilder};
    use crate::test_utils::MockDefault;
    use crate::types::{BgnDe, CorpCode};
    use crate::{subscribe_tracing_with_span, test_context, TestContext};
    use goldrust::Content;

    #[tokio::test]
    #[tracing::instrument]
    async fn get_list_default() {
        subscribe_tracing_with_span!("test");
        let mut ctx = test_context!().await;

        ctx.arrange_test_endpoint::<List>("/api/list.json").await;

        // region: Action
        let params = ListRequestParamsBuilder::default()
            .corp_code(CorpCode::mock_default())
            .bgn_de(BgnDe::mock_default())
            .build()
            .expect("Failed to build ListRequestParams");
        tracing::debug!(?params, "Request parameters");

        let response = ctx
            .api
            .get_list(params)
            .await
            .expect("get_list should succeed");
        tracing::info!(?response, "Got response");
        // endregion

        // region: Assert
        assert!(
            response.status().is_success(),
            "Response didn't return a status code of 2xx"
        );
        // endregion

        // region: Save response body
        ctx.goldrust
            .save(Content::Json(
                serde_json::to_value(response.body)
                    .expect("Failed to convert to serde_json::Value"),
            ))
            .expect("Failed to save response body");
        // endregion
    }
}
