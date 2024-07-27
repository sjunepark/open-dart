use std::fmt::Display;

use reqwest;
use reqwest::IntoUrl;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::endpoints::{List, ListRequestParams, OpenDartResponse};
use crate::error::{map_deserialization_error, DeserializationError, OpenDartError};
use crate::types::ResponseType;

pub struct OpenDartApi {
    client: reqwest::Client,
    config: OpenDartConfig,
}

#[derive(Clone)]
pub struct OpenDartConfig {
    /// API version to use
    pub api_version: u32,
    /// OpenDart API key from [OpenDart](https://opendart.fss.or.kr/)
    pub api_key: String,
}

impl OpenDartApi {
    pub fn new(config: OpenDartConfig) -> Self {
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
        response_type: ResponseType,
    ) -> Result<OpenDartResponse<List>, OpenDartError> {
        self.get("https://opendart.fss.or.kr/api/list", args, response_type)
            .await
    }

    // endregion: Public APIs

    // region: Generic APIs

    async fn get<'de, U, P, R>(
        &self,
        url: U,
        params: P,
        response_type: ResponseType,
    ) -> Result<OpenDartResponse<R>, OpenDartError>
    where
        U: Display + IntoUrl,
        P: Serialize,
        R: DeserializeOwned + Validate,
    {
        let url = format!("{}.{}", url, response_type);
        let request = self.client.get(url).query(&params).build()?;
        let response = self.client.execute(request).await?;
        let bytes = response.bytes().await?;
        let response: OpenDartResponse<R> = match response_type {
            ResponseType::Json => serde_json::from_slice(&bytes)
                .map_err(|e| map_deserialization_error(DeserializationError::from(e), &bytes))?,
            ResponseType::Xml => quick_xml::de::from_reader(bytes.as_ref())
                .map_err(|e| map_deserialization_error(DeserializationError::from(e), &bytes))?,
        };
        response.validate()?;
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
    use test_log::test;
    use validator::Validate;

    use crate::endpoints::ListRequestParamsBuilder;
    use crate::types::ResponseType;
    use crate::TestContext;

    #[test(tokio::test)]
    async fn test_get_list_json() {
        let api = TestContext::new().api;
        let params = ListRequestParamsBuilder::default().build().unwrap();
        let response = api.get_list(params, ResponseType::Json).await.unwrap();
        assert!(response.validate().is_ok())
    }

    #[test(tokio::test)]
    async fn test_get_list_xml() {
        let api = TestContext::new().api;
        let params = ListRequestParamsBuilder::default().build().unwrap();
        let response = api.get_list(params, ResponseType::Xml).await.unwrap();
        assert!(response.validate().is_ok())
    }
}
