use serde::Deserialize;
use validator::{Validate, ValidationError, ValidationErrors};

pub use list::*;

use crate::types::CrtfcKey;

mod list;

pub trait OpenDartApiKey {
    fn open_dart_api_key() -> CrtfcKey {
        std::env::var("OPEN_DART_API_KEY")
            .expect("OPEN_DART_API_KEY must be set as an environment variable")
            .into()
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct Message {
    /// ### 에러 및 정보 코드
    /// (※메시지 설명 참조)
    #[validate(custom(function = "Self::validate_status"))]
    pub status: String,

    /// ### 에러 및 정보 메시지
    /// (※메시지 설명 참조)
    pub message: String,
}

impl Message {
    /// ### 메시지 설명
    /// - 000 :정상
    /// - 010 :등록되지 않은 키입니다.
    /// - 011 :사용할 수 없는 키입니다. 오픈API에 등록되었으나, 일시적으로 사용 중지된 키를 통하여 검색하는 경우 발생합니다.
    /// - 012 :접근할 수 없는 IP입니다.
    /// - 013 :조회된 데이타가 없습니다.
    /// - 014 :파일이 존재하지 않습니다.
    /// - 020 :요청 제한을 초과하였습니다. 일반적으로는 20,000건 이상의 요청에 대하여 이 에러 메시지가 발생되나, 요청 제한이 다르게 설정된 경우에는 이에 준하여 발생됩니다.
    /// - 021 :조회 가능한 회사 개수가 초과하였습니다.(최대 100건)
    /// - 100 :필드의 부적절한 값입니다. 필드 설명에 없는 값을 사용한 경우에 발생하는 메시지입니다.
    /// - 101 :부적절한 접근입니다.
    /// - 800 :시스템 점검으로 인한 서비스가 중지 중입니다.
    /// - 900 :정의되지 않은 오류가 발생하였습니다.
    /// - 901 :사용자 계정의 개인정보 보유기간이 만료되어 사용할 수 없는 키입니다. 관리자 이메일(opendart@fss.or.kr)로 문의하시기 바랍니다
    fn validate_status(status: &str) -> Result<(), ValidationError> {
        match status {
            "000" | "013" => Ok(()),
            "010" | "011" | "012" | "014" | "020" | "021" | "100" | "101" | "800" | "900"
            | "901" => Err(ValidationError::new(
                "error status received from OpenDart API",
            )),
            _ => Err(ValidationError::new(
                "unknown status received from OpenDart API",
            )),
        }
    }
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "status: {}, message: {}", self.status, self.message)
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum OpenDartResponse<T: Validate> {
    Json(JsonResponse<T>),
    Xml(XmlResponse<T>),
}

impl<T: Validate> Validate for OpenDartResponse<T> {
    fn validate(&self) -> Result<(), ValidationErrors> {
        match self {
            OpenDartResponse::Json(json) => json.validate(),
            OpenDartResponse::Xml(xml) => xml.validate(),
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct JsonResponse<T: Validate> {
    #[serde(flatten)]
    #[validate(nested)]
    pub message: Message,

    #[serde(flatten)]
    #[validate(nested)]
    pub content: Option<T>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct XmlResponse<T: Validate> {
    pub result: JsonResponse<T>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_validate_status_ok() {
        let message = Message {
            status: "000".into(),
            message: "hello".into(),
        };
        assert!(message.validate().is_ok());
    }

    #[test]
    fn can_fail_validate_status_known() {
        let message = Message {
            status: "010".into(),
            message: "hello".into(),
        };
        assert!(message.validate().is_err());
    }

    #[test]
    fn can_fail_validate_status_unknown() {
        let message = Message {
            status: "999".into(),
            message: "hello".into(),
        };
        assert!(message.validate().is_err());
    }
}
