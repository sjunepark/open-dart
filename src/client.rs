use derive_builder::Builder;
use reqwest;
use reqwest::IntoUrl;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Display;

use crate::endpoints::OpenDartResponse;
use crate::error::OpenDartError;

#[derive(Debug)]
pub struct OpenDartApi {
    client: reqwest::Client,
    config: OpenDartConfig,
}

impl OpenDartApi {
    pub(crate) fn url(&self, path: &str) -> String {
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

    // endregion

    // region: Generic APIs
    #[tracing::instrument(skip(self))]
    pub(crate) async fn get<'de, U, P, B>(
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
