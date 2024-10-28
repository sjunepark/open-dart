#![cfg(test)]

use crate::client::{OpenDartApi, OpenDartConfigBuilder};

use goldrust::{goldrust, Goldrust, ResponseSource};
use serde::de::DeserializeOwned;
use serde::Serialize;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

macro_rules! test_context {
    () => {
        $crate::test_utils::TestContext::new($crate::function_id!())
    };
}
pub(crate) use test_context;

#[derive(Debug)]
pub struct TestContext {
    function_id: String,
    pub api: OpenDartApi,
    pub mock_server: wiremock::MockServer,
    pub goldrust: Goldrust,
}

impl TestContext {
    pub async fn new(function_id: &str) -> Self {
        let goldrust = goldrust!("json", function_id);

        // region: Mock server setup
        let mock_server = wiremock::MockServer::start().await;
        // endregion

        // region: Goldrust setup
        let domain = match goldrust.response_source {
            ResponseSource::Local => mock_server.uri(),
            ResponseSource::External => "https://opendart.fss.or.kr".to_string(),
        };

        // endregion

        // region: Initialize OpenDartApi
        let config = OpenDartConfigBuilder::default()
            .domain(domain)
            .build()
            .expect("Failed to build OpenDartConfig");
        let api = OpenDartApi::with_config(config);
        // endregion

        Self {
            function_id: function_id.to_string(),
            api,
            mock_server,
            goldrust,
        }
    }

    #[tracing::instrument(skip(self))]
    pub async fn arrange_test_endpoint<R>(&mut self, api_path: &str)
    where
        R: Serialize + DeserializeOwned + std::fmt::Debug,
    {
        let response_source = &self.goldrust.response_source;
        let golden_file_path = &self.goldrust.golden_file_path;

        // region: Arrange
        match response_source {
            ResponseSource::Local => {
                tracing::debug!(
                    ?response_source,
                    ?golden_file_path,
                    "Getting response body from file"
                );
                let golden_file_str = std::fs::read_to_string(golden_file_path)
                    .expect("Failed to read response body from file");
                let golden_file_body: R = serde_json::from_str(&golden_file_str)
                    .expect("Failed to deserialize response body");

                let response = ResponseTemplate::new(200).set_body_json(&golden_file_body);

                Mock::given(method("GET"))
                    .and(path(api_path))
                    .respond_with({
                        tracing::debug!(?golden_file_body, "Responding from mock server");
                        response
                    })
                    .mount(&self.mock_server)
                    .await;
            }
            ResponseSource::External => {
                tracing::debug!(?response_source, "Getting response body from external API");
            }
        }
        // endregion
    }
}

pub trait MockDefault: Sized {
    fn mock_default() -> Self;
}

#[cfg(test)]
pub(crate) mod tracing_setup {
    use tracing_subscriber::EnvFilter;

    pub fn subscribe() {
        let subscriber = tracing_subscriber::fmt()
            .with_test_writer()
            .with_env_filter(EnvFilter::from_default_env())
            .pretty()
            .finish();

        tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");
    }

    macro_rules! subscribe_tracing_with_span {
        ($span_name:expr) => {
            $crate::test_utils::tracing_setup::subscribe();

            let function_id = $crate::function_id!();
            let _span = tracing::info_span!($span_name, ?function_id).entered();
        };
    }
    pub(crate) use subscribe_tracing_with_span;
}

#[macro_export]
macro_rules! function_id {
    () => {{
        fn f() {}
        fn type_name_of_val<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let mut name = type_name_of_val(f).strip_suffix("::f").unwrap_or("");
        while let Some(rest) = name.strip_suffix("::{{closure}}") {
            name = rest;
        }
        &name.replace("::", "-")
    }};
}

mod tests {
    use super::*;

    #[tokio::test]
    async fn test_context_function_id_should_be_as_expected() {
        let ctx = test_context!().await;
        let expected = function_id!();
        assert_eq!(
            ctx.function_id, *expected,
            "function_id should be as expected"
        );
    }
}
