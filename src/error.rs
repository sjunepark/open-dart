use derive_builder::UninitializedFieldError;
use std::str::Utf8Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OpenDartError {
    /// Error when a response cannot be deserialized into a Rust type
    #[error("serde_json deserialization error: {0}")]
    Deserialize(#[from] serde_json::Error),
    #[error("chrono format parse error: {0}")]
    ChronoParse(#[from] chrono::format::ParseError),
    /// Underlying error from the reqwest library after an API call was made
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    /// Error from client side validation
    /// or when the builder fails to build request before making API call
    #[error("derive_builder uninitialized field error: {0}")]
    UninitializedField(#[from] UninitializedFieldError),
    #[error("utf8 error: {0}")]
    Utf8(#[from] Utf8Error),
    #[error("validation error: {0}")]
    Validation(#[from] ValidationError),
}

#[derive(
    std::fmt::Debug,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    // derive_more
    derive_more::From,
    derive_more::Into,
    // serde
    serde::Serialize,
    serde::Deserialize,
    // thiserror
    Error,
)]
#[error("value: {value}, message: {message}")]
pub struct ValidationError {
    pub value: String,
    pub message: String,
}
