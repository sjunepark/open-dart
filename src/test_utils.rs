#![cfg(test)]

use serde::Serialize;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tracing::metadata::LevelFilter;
use tracing_log::AsLog;
use tracing_subscriber::EnvFilter;

use crate::client::{OpenDartApi, OpenDartConfigBuilder};
use crate::endpoints::OpenDartResponseBody;

pub struct TestContext {
    pub api: OpenDartApi,
    pub mock_server: wiremock::MockServer,
    pub update_golden_files: bool,
}

impl TestContext {
    pub async fn new() -> Self {
        // region: Tracing setup
        let subscriber = tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .pretty()
            .finish();

        let _ = tracing::subscriber::set_global_default(subscriber);

        let current_level = LevelFilter::current();

        let _ = tracing_log::LogTracer::builder()
            // Note that we must call this *after* setting the global default subscriber
            // so that we get its max level hint.
            .with_max_level(current_level.as_log())
            .init();
        // endregion: Tracing setup

        // region: Mock server setup
        let mock_server = wiremock::MockServer::start().await;
        // endregion: Mock server setup

        // region: Set domain for external open dart api calls
        let allow_external_call: bool = std::env::var("ALLOW_EXTERNAL_API_CALL")
            .expect("ALLOW_EXTERNAL_API_CALL must be set")
            .parse()
            .expect(r#"ALLOW_EXTERNAL_API_CALL must be a boolean, e.g., "true" or "false""#);

        let domain = if allow_external_call {
            "https://opendart.fss.or.kr".to_string()
        } else {
            mock_server.uri()
        };
        // endregion: Set domain for external open dart api calls

        // region: Initialize OpenDartApi
        let config = OpenDartConfigBuilder::default()
            .domain(domain)
            .build()
            .expect("Failed to build OpenDartConfig");
        let api = OpenDartApi::new(config);
        // endregion: Initialize OpenDartApi

        // region: Update flag setup
        let update_golden_files: bool = std::env::var("UPDATE_GOLDEN_FILES")
            .expect("UPDATE_GOLDEN_FILES must be set")
            .parse()
            .expect(r#"UPDATE_GOLDEN_FILES must be a boolean, e.g., "true" or "false""#);
        // endregion: Update flag setup

        Self {
            api,
            mock_server,
            update_golden_files,
        }
    }
}

pub async fn save_response_body<R: Serialize>(
    body: OpenDartResponseBody<R>,
    path: &str,
) -> anyhow::Result<()> {
    let bytes = serde_json::to_string_pretty(&body).expect("Failed to serialize response body");
    let bytes = bytes.as_bytes();
    let mut file = File::create(path).await.expect("Failed to create file");
    file.write_all(bytes).await.map_err(anyhow::Error::from)
}

pub fn get_test_name() -> String {
    std::thread::current()
        .name()
        .expect("Test name must be set")
        .split("::")
        .last()
        .expect("Failed to get last element within test name parts")
        .to_string()
}
