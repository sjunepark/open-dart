use derive_builder::UninitializedFieldError;

#[derive(Debug, thiserror::Error)]
pub enum OpenDartError {
    /// Underlying error from reqwest library after an API call was made
    #[error("http error: {0}")]
    Reqwest(#[from] reqwest::Error),
    /// Error when a response cannot be deserialized into a Rust type
    #[error("failed to deserialize api response: {0}")]
    JSONDeserialize(#[from] serde_json::Error),
    /// Error from client side validation
    /// or when builder fails to build request before making API call
    #[error("invalid args: {0}")]
    InvalidArgument(String),
}

impl From<UninitializedFieldError> for OpenDartError {
    fn from(value: UninitializedFieldError) -> Self {
        OpenDartError::InvalidArgument(value.to_string())
    }
}
