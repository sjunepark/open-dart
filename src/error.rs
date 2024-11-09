use crate::endpoints::Message;
use derive_builder::UninitializedFieldError;
use reqwest::StatusCode;
use std::str::Utf8Error;
use thiserror::Error;
use validator::ValidationError;

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
    /// Error when a response is not successful.
    ///
    /// This will usually appear when a 200 status code was received,
    /// but the response body indicates an error.
    #[error("message error: {0}")]
    Message(#[from] MessageError),
    #[error("quick_xml error: {0}")]
    QuickXml(#[from] quick_xml::errors::Error),
    #[error("response error: {0}")]
    Response(#[from] ResponseError),
    #[error("unexpected zip content error: {0}")]
    UnexpectedZipContent(#[from] UnexpectedZipContentError),
    /// Error from client side validation
    /// or when the builder fails to build request before making API call
    #[error("derive_builder uninitialized field error: {0}")]
    UninitializedField(#[from] UninitializedFieldError),
    #[error("utf8 error: {0}")]
    Utf8(#[from] Utf8Error),
    // todo: remove
    #[error("validation error: {0}")]
    MyValidation(#[from] MyValidationError),
    #[error("validation error: {0}")]
    Validator(#[from] ValidationError),
    #[error("zip error: {0}")]
    Zip(#[from] zip::result::ZipError),
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
#[error("{self:?}")]
pub struct MyValidationError {
    pub value: String,
    pub message: String,
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
#[error("{self:?}")]
pub struct MessageError {
    pub message: Message,
}

#[derive(
    std::fmt::Debug,
    Clone,
    Eq,
    PartialEq,
    // derive_more
    derive_more::From,
    derive_more::Into,
    // thiserror
    Error,
)]
#[error("{self:?}")]
pub struct ResponseError {
    pub status: StatusCode,
    pub headers: reqwest::header::HeaderMap,
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
#[error("{self:?}")]
pub struct UnexpectedZipContentError {
    pub files: Vec<String>,
}
