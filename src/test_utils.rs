#![cfg(test)]

use tracing::metadata::LevelFilter;
use tracing_log::AsLog;
use tracing_subscriber::EnvFilter;

use crate::client::{OpenDartApi, OpenDartConfig};

pub struct TestContext {
    pub api: OpenDartApi,
}

impl TestContext {
    pub fn new() -> Self {
        let subscriber = tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .pretty()
            .finish();

        let _ = tracing::subscriber::set_global_default(subscriber);

        let current_level = LevelFilter::current();

        let _ = tracing_log::LogTracer::builder()
            // Note that we must call this *after* setting the global default
            // subscriber, so that we get its max level hint.
            .with_max_level(current_level.as_log())
            .init();

        let api = OpenDartApi::new(OpenDartConfig { api_version: 1 });

        Self { api }
    }
}

impl Default for TestContext {
    fn default() -> Self {
        Self::new()
    }
}
