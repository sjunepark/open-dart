use crate::endpoints::base::ResponseBody;
use crate::endpoints::{OpenDartResponse, ResponseCheck};
use crate::error::{OpenDartError, ResponseError};
use bytes::Bytes;
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
    ) -> Result<OpenDartResponse<ResponseBody<B>>, OpenDartError>
    where
        U: Display + IntoUrl + std::fmt::Debug,
        P: Serialize + std::fmt::Debug,
        B: Serialize + ResponseCheck + DeserializeOwned + std::fmt::Debug,
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

        let json_deserializer = &mut serde_json::Deserializer::from_slice(&bytes);
        let response_body: Option<ResponseBody<B>> =
            serde_path_to_error::deserialize(json_deserializer)
                .inspect_err(|e| {
                    tracing::error!(?e, body = ?text, "Failed to deserialize response body");
                })
                .unwrap();

        // The deserialization type should be an `Option`
        // because there can be no body in the case of an unsuccessful response
        // let response_body =
        //     serde_json::from_slice::<Option<ResponseBody<B>>>(&bytes).inspect_err(|_e| {
        //         tracing::error!(body = ?text, "Failed to deserialize response body");
        //     })?;

        if let Some(body) = &response_body {
            body.is_success()?;
        }

        let response = OpenDartResponse::new(status, headers, response_body);
        Ok(response)
    }

    #[tracing::instrument(skip(self))]
    pub(crate) async fn get_zip<U>(&self, url: U) -> Result<Bytes, OpenDartError>
    where
        U: Display + IntoUrl + std::fmt::Debug,
    {
        #[derive(Debug, Serialize)]
        struct Params {
            crtfc_key: String,
        }
        let params = Params {
            crtfc_key: std::env::var("OPEN_DART_API_KEY")
                .expect("OPEN_DART_API_KEY must be set as an environment variable"),
        };
        let request = self.client.get(url).query(&params).build()?;

        let response = self.client.execute(request).await?;

        let headers = response.headers().clone();
        let status = response.status();

        if status != reqwest::StatusCode::OK {
            tracing::error!(?status, ?headers, "Failed to get zip file");
            Err(ResponseError { status, headers })?;
        }

        let bytes = response
            .bytes()
            .await
            .inspect_err(|_e| tracing::error!("Failed to parse response body as bytes"))?;

        Ok(bytes)
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
