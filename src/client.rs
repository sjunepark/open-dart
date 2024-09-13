use std::fmt::Display;

use reqwest;
use reqwest::IntoUrl;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::endpoints::{List, ListRequestParams, OpenDartResponse};
use crate::error::{map_deserialization_error, OpenDartError};

#[allow(dead_code)]
pub struct OpenDartApi {
    client: reqwest::Client,
    config: OpenDartConfig,
}

#[derive(Clone)]
pub struct OpenDartConfig {
    /// API version to use
    pub api_version: u32,
}

impl Default for OpenDartApi {
    /// Create a new `OpenDartApi` instance with default configuration.
    ///
    /// The default configuration is as below:
    /// - `api_version`: 1
    /// - `api_key`: Loaded from the `OPEN_DART_API_KEY` environment variable
    fn default() -> Self {
        let api_version = 1;
        let config = OpenDartConfig { api_version };

        Self::new(config)
    }
}

impl OpenDartApi {
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

    // region: Public APIs

    pub async fn get_list(
        &self,
        args: ListRequestParams,
    ) -> Result<OpenDartResponse<List>, OpenDartError> {
        self.get("https://opendart.fss.or.kr/api/list.json", args)
            .await
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
        let bytes = response.bytes().await?;
        let response: OpenDartResponse<R> =
            serde_json::from_slice(&bytes).map_err(|e| map_deserialization_error(e, &bytes))?;
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

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn test_get_list_default() {
        // todo: Mock the request
    }
}
