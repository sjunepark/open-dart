use crate::client::{OpenDartApi, OpenDartConfig};
use crate::config::Settings;

pub struct TestContext {
    pub api: OpenDartApi,
}

impl TestContext {
    pub fn new() -> Self {
        let settings = Settings::new().unwrap();
        std::env::set_var("OPEN_DART_API_KEY", &settings.open_dart_api_key);

        let api = OpenDartApi::new(OpenDartConfig {
            api_version: 1,
            api_key: settings.open_dart_api_key,
        });

        Self { api }
    }
}
