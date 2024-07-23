use anyhow::Result;
use reqwest;

use crate::endpoints::{ListRequestParams, ListRequestParamsBuilder};

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

    pub async fn get_list(&self, args: ListRequestParams) -> Result<()> {
        let url = "https://opendart.fss.or.kr/api/list.json";

        println!("{:?}", args);
        let request = self.client.get(url).query(&args).build()?;
        println!("{:?}", request);
        let response = self.client.execute(request).await?;
        println!("{:?}", response);
        let a = response.text().await?;
        println!("{:?}", a);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::TestContext;

    use super::*;

    #[tokio::test]
    async fn test_get_list() {
        let api = TestContext::new().api;
        let args = ListRequestParamsBuilder::default().build().unwrap();

        api.get_list(args).await.unwrap();
    }
}
