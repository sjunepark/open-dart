use derive_more::Display;
use serde::{Deserialize, Serialize};

use crate::assert_impl_commons_without_default;

assert_impl_commons_without_default!(ReportNm);

/// ### 보고서명
/// 공시구분+보고서명+기타정보
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
    // serde
    Serialize,
    Deserialize,
)]
pub enum ReportNm {
    /// 기재정정: 본 보고서명으로 이미 제출된 보고서의 기재내용이 변경되어 제출된 것임
    Amendment,
    /// 첨부정정: 본 보고서명으로 이미 제출된 보고서의 첨부내용이 변경되어 제출된 것임
    AttachmentRevision,
    /// 첨부추가: 본 보고서명으로 이미 제출된 보고서의 첨부서류가 추가되어 제출된 것임
    AdditionalAttachment,
    /// 변경등록: 본 보고서명으로 이미 제출된 보고서의 유동화계획이 변경되어 제출된 것임
    PlanModification,
    /// 연장결정: 본 보고서명으로 이미 제출된 보고서의 신탁계약이 연장되어 제출된 것임
    ContractExtension,
    /// 발행조건확정: 본 보고서명으로 이미 제출된 보고서의 유가증권 발행조건이 확정되어 제출된 것임
    IssuanceConditionFinalized,
    /// 정정명령부과: 본 보고서에 대하여 금융감독원이 정정명령을 부과한 것임
    CorrectionOrder,
    /// 정정제출요구: 본 보고서에 대하여 금융감독원이 정정제출요구을 부과한 것임
    SubmissionRevisionRequest,
}

#[cfg(test)]
impl crate::test_utils::MockDefault for ReportNm {
    fn mock_default() -> Self {
        Self::Amendment
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let report_nma = ReportNm::Amendment;
        let serialized = serde_json::to_string(&report_nma).expect("Failed to serialize");
        assert_eq!(serialized, r#""Amendment""#);
    }

    #[test]
    fn deserialize() {
        let report_nma = ReportNm::Amendment;
        let deserialized: ReportNm =
            serde_json::from_str(r#""Amendment""#).expect("Failed to deserialize");
        assert_eq!(deserialized, report_nma);
    }

    #[test]
    fn display() {
        assert_eq!(ReportNm::Amendment.to_string(), "Amendment");
    }
}
