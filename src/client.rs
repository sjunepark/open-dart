use derive_builder::Builder;
use reqwest;
use reqwest::IntoUrl;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Display;

use crate::endpoints::list;
use crate::endpoints::OpenDartResponse;
use crate::error::OpenDartError;

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
    pub(crate) fn with_config(config: OpenDartConfig) -> Self {
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
        args: list::Params,
    ) -> Result<OpenDartResponse<list::ResponseBody>, OpenDartError> {
        self.get(self.url("/api/list.json"), args).await
    }

    // endregion

    // region: Generic APIs
    #[tracing::instrument(skip(self))]
    async fn get<'de, U, P, B>(
        &self,
        url: U,
        params: P,
    ) -> Result<OpenDartResponse<B>, OpenDartError>
    where
        U: Display + IntoUrl + std::fmt::Debug,
        P: Serialize + std::fmt::Debug,
        B: Serialize + DeserializeOwned + std::fmt::Debug,
    {
        let request = self.client.get(url).query(&params).build()?;
        let response = self.client.execute(request).await?;

        let headers = response.headers().clone();
        let status = response.status();

        let bytes = response
            .bytes()
            .await
            .inspect_err(|_e| tracing::error!("Failed to parse response body as bytes"))?;
        // For debugging
        let text = std::str::from_utf8(&bytes).inspect_err(|_e| {
            tracing::error!("Failed to parse response body as text");
        })?;

        let response_body = serde_json::from_slice::<Option<B>>(&bytes).inspect_err(|_e| {
            tracing::error!(body = ?text, "Failed to deserialize response body");
        })?;

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
    /// - `domain`: <https://opendart.fss.or.kr>
    fn default() -> Self {
        Self::with_config(OpenDartConfig::default())
    }
}

#[derive(Builder, Clone, Debug)]
#[builder(default)]
pub(crate) struct OpenDartConfig {
    /// The domain, which will default to <https://opendart.fss.or.kr>
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
    use crate::endpoints::list;
    use crate::test_utils::MockDefault;
    use crate::types::{BgnDe, CorpCode};
    use crate::{subscribe_tracing_with_span, test_context, TestContext};
    use goldrust::Content;

    #[tokio::test]
    #[tracing::instrument]
    async fn get_list_default() {
        subscribe_tracing_with_span!("test");
        let mut ctx = test_context!().await;

        ctx.arrange_test_endpoint::<list::ResponseBody>("/api/list.json")
            .await;

        // region: Action
        let params = list::ParamsBuilder::default()
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
