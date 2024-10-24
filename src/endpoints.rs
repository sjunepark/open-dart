use crate::types::CrtfcKey;
use derive_more::Display;
use reqwest::{header::HeaderMap, StatusCode};
use serde::{Deserialize, Serialize};

mod list;

pub use list::{List, ListRequestParams, ListRequestParamsBuilder};

pub trait OpenDartApiKey {
    fn open_dart_api_key() -> CrtfcKey {
        CrtfcKey::default()
    }
}

#[derive(Debug, Serialize, Deserialize, Display)]
#[display("status: {status}, message: {message}")]
pub struct Message {
    /// ### 에러 및 정보 코드
    /// (※메시지 설명 참조)
    pub status: String,

    /// ### 에러 및 정보 메시지
    /// (※메시지 설명 참조)
    pub message: String,
}

#[derive(Debug)]
pub struct OpenDartResponse<R>
where
    R: Serialize,
{
    status: StatusCode,
    header_map: HeaderMap,
    pub body: OpenDartResponseBody<R>,
}

impl<R: Serialize> OpenDartResponse<R> {
    pub fn new(status: StatusCode, header_map: HeaderMap, body: OpenDartResponseBody<R>) -> Self {
        Self {
            status,
            header_map,
            body,
        }
    }

    pub fn status(&self) -> StatusCode {
        self.status
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenDartResponseBody<R> {
    #[serde(flatten)]
    pub message: Message,

    #[serde(flatten)]
    pub content: Option<R>,
}

impl<R> OpenDartResponseBody<R> {}
