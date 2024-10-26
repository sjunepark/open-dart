use crate::assert_impl_commons_without_default;
use crate::error::{OpenDartError, ValidationError};

use derive_more::{AsRef, Display, From, Into};
use serde::{Deserialize, Serialize};

/// ### 보고서명
/// 공시구분+보고서명+기타정보
///
/// - 기재정정: 본 보고서명으로 이미 제출된 보고서의 기재내용이 변경되어 제출된 것임
/// - 첨부정정: 본 보고서명으로 이미 제출된 보고서의 첨부내용이 변경되어 제출된 것임
/// - 첨부추가: 본 보고서명으로 이미 제출된 보고서의 첨부서류가 추가되어 제출된 것임
/// - 변경등록: 본 보고서명으로 이미 제출된 보고서의 유동화계획이 변경되어 제출된 것임
/// - 연장결정: 본 보고서명으로 이미 제출된 보고서의 신탁계약이 연장되어 제출된 것임
/// - 발행조건확정: 본 보고서명으로 이미 제출된 보고서의 유가증권 발행조건이 확정되어 제출된 것임
/// - 정정명령부과: 본 보고서에 대하여 금융감독원이 정정명령을 부과한 것임
/// - 정정제출요구: 본 보고서에 대하여 금융감독원이 정정제출요구을 부과한 것임
#[derive(
    Debug,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    // derive_more
    AsRef,
    Display,
    From,
    Into,
    // serde
    Serialize,
    Deserialize,
)]
#[cfg_attr(
    feature = "diesel_newtype",
    derive(diesel_derive_newtype::DieselNewType)
)]
pub struct ReportNm(String);
assert_impl_commons_without_default!(ReportNm);

impl ReportNm {
    pub fn try_new(value: &str) -> Result<Self, OpenDartError> {
        if value.is_empty() {
            return Err(ValidationError {
                value: value.to_string(),
                message: "empty string is not allowed".to_string(),
            })?;
        };
        Ok(Self(value.to_string()))
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

#[cfg(test)]
impl crate::test_utils::MockDefault for ReportNm {
    fn mock_default() -> Self {
        let name = "증권발행실적보고서".to_string();
        ReportNm::try_new(&name)
            .unwrap_or_else(|_| panic!("failed to create ReportNm with: {}", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let report_nm =
            ReportNm::try_new("증권발행실적보고서").expect("failed to create report_nm");
        let serialized = serde_json::to_string(&report_nm).expect("failed to serialize");
        assert_eq!(serialized, "\"증권발행실적보고서\"");
    }

    #[test]
    fn deserialize() {
        let report_nm = serde_json::from_str::<ReportNm>("\"증권발행실적보고서\"")
            .expect("failed to deserialize");
        assert_eq!(report_nm.into_inner(), "증권발행실적보고서");
    }

    #[test]
    fn try_new_with_empty_string_should_fail() {
        let report_nm = ReportNm::try_new("");
        assert!(report_nm.is_err());
    }
}
