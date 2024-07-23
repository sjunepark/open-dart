pub trait OpenDartApiKey {
    fn open_dart_api_key() -> String {
        std::env::var("OPEN_DART_API_KEY")
            .expect("OPEN_DART_API_KEY must be set as an environment variable")
    }
}
