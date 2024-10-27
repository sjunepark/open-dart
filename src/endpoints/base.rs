use crate::assert_impl_commons_without_default;
use crate::types::CrtfcKey;
use derive_more::{Display, From, Into};
use reqwest::header::HeaderMap;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

pub trait OpenDartApiKey {
    fn open_dart_api_key() -> CrtfcKey {
        CrtfcKey::default()
    }
}

#[derive(
    Debug,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    // derive_more
    Display,
    From,
    Into,
    // serde
    Serialize,
    Deserialize,
)]
#[display("status: {status}, message: {message}")]
pub struct Message {
    /// ### 에러 및 정보 코드
    /// (※메시지 설명 참조)
    pub status: String,

    /// ### 에러 및 정보 메시지
    /// (※메시지 설명 참조)
    pub message: String,
}
assert_impl_commons_without_default!(Message);

#[derive(Debug)]
pub struct OpenDartResponse<B>
where
    B: Serialize,
{
    status: StatusCode,
    _header_map: HeaderMap,
    pub body: Option<B>,
}

impl<B: Serialize> OpenDartResponse<B> {
    pub fn new(status: StatusCode, header_map: HeaderMap, body: Option<B>) -> Self {
        Self {
            status,
            _header_map: header_map,
            body,
        }
    }

    pub fn status(&self) -> StatusCode {
        self.status
    }
}
