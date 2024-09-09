use serde::Deserialize;

use crate::types::CrtfcKey;

mod list;

pub use list::{List, ListRequestParams, ListRequestParamsBuilder};

pub trait OpenDartApiKey {
    fn open_dart_api_key() -> CrtfcKey {
        CrtfcKey::default()
    }
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum OpenDartResponse<T> {
    Json(JsonResponse<T>),
    Xml(XmlResponse<T>),
}

#[derive(Debug, Deserialize)]
pub struct JsonResponse<T> {
    #[serde(flatten)]
    pub message: Message,

    #[serde(flatten)]
    pub content: Option<T>,
}

#[derive(Debug, Deserialize)]
pub struct XmlResponse<T> {
    pub result: JsonResponse<T>,
}
