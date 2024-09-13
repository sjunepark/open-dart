use crate::types::CrtfcKey;
use reqwest::{header::HeaderMap, StatusCode};
use serde::{Deserialize, Serialize};

mod list;

pub use list::{List, ListRequestParams, ListRequestParamsBuilder};

pub trait OpenDartApiKey {
    fn open_dart_api_key() -> CrtfcKey {
        CrtfcKey::default()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    /// ### 에러 및 정보 코드
    /// (※메시지 설명 참조)
    pub status: String,

    /// ### 에러 및 정보 메시지
    /// (※메시지 설명 참조)
    pub message: String,
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "status: {}, message: {}", self.status, self.message)
    }
}

#[derive(Debug)]
pub struct OpenDartResponse<R>
where
    R: Serialize,
{
    status: StatusCode,
    header_map: HeaderMap,
    body: OpenDartResponseBody<R>,
}

impl<R: Serialize> OpenDartResponse<R> {
    pub fn new(status: StatusCode, header_map: HeaderMap, body: OpenDartResponseBody<R>) -> Self {
        Self {
            status,
            header_map,
            body,
        }
    }

    pub fn headers(&self) -> &HeaderMap {
        &self.header_map
    }

    pub fn status(&self) -> StatusCode {
        self.status
    }

    pub fn body(&self) -> &OpenDartResponseBody<R> {
        &self.body
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenDartResponseBody<R> {
    #[serde(flatten)]
    pub message: Message,

    #[serde(flatten)]
    pub content: Option<R>,
}
