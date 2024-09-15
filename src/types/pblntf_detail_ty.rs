use crate::assert_impl_commons_without_default;
use derive_more::{AsMut, AsRef, Display};
use generate_consts::generate_consts;
use serde::{Deserialize, Serialize};

assert_impl_commons_without_default!(PblntfDetailTy);

/// ### 공시상세유형
/// The detailed documentation exists on each constants(A001, A002, ...).
///
/// A: 정기공시
/// B: 주요사항보고
/// C: 발행공시
/// D: 지분공시
/// E: 기타공시
/// F: 외부감사관련
/// G: 펀드공시
/// H: 자산유동화
/// I: 거래소공시
/// J: 공정위공시
#[derive(
    Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Display, Serialize, Deserialize, AsMut, AsRef,
)]
pub struct PblntfDetailTy(Inner);

#[cfg(test)]
use crate::test_utils::MockDefault;

#[cfg(test)]
impl MockDefault for PblntfDetailTy {
    fn mock_default() -> Self {
        Self(Inner::F001)
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Display, Serialize, Deserialize)]
#[display("{_variant}")]
#[generate_consts(PblntfDetailTy)]
enum Inner {
    // region A 정기공시
    /// 사업보고서
    A001,
    /// 반기보고서
    A002,
    /// 분기보고서
    A003,
    /// 등록법인결산서류자본시장법이전
    A004,
    /// 소액공모법인결산서류
    // endregion
    A005,

    // region B 주요사항보고
    /// 주요사항보고서
    B001,
    /// 주요경영사항신고자본시장법 이전
    B002,
    /// 최대주주등과의거래신고자본시장법 이전
    B003,
    // endregion

    // region C 발행공시
    /// 증권신고지분증권
    C001,
    /// 증권신고채무증권
    C002,
    /// 증권신고파생결합증권
    C003,
    /// 증권신고합병등
    C004,
    /// 증권신고기타
    C005,
    /// 소액공모지분증권
    C006,
    /// 소액공모채무증권
    C007,
    /// 소액공모파생결합증권
    C008,
    /// 소액공모합병등
    C009,
    /// 소액공모기타
    C010,
    /// 호가중개시스템을통한소액매출
    C011,
    // endregion

    // region D 지분공시
    /// 주식등의대량보유상황보고서
    D001,
    /// 임원ㆍ주요주주특정증권등소유상황보고서
    D002,
    /// 의결권대리행사권유
    D003,
    /// 공개매수
    D004,
    /// 임원ㆍ주요주주 특정증권등 거래계획보고서
    D005,
    // endregion

    // region E 기타공시
    /// 자기주식취득처분
    E001,
    /// 신탁계약체결해지
    E002,
    /// 합병등종료보고서
    E003,
    /// 주식매수선택권부여에관한신고
    E004,
    /// 사외이사에관한신고
    E005,
    /// 주주총회소집보고서
    E006,
    /// 시장조성안정조작
    E007,
    /// 합병등신고서자본시장법 이전
    E008,
    /// 금융위등록취소자본시장법 이전
    E009,
    // endregion

    // region F 외부감사관련
    /// 감사보고서
    F001,
    /// 연결감사보고서
    F002,
    /// 결합감사보고서
    F003,
    /// 회계법인사업보고서
    F004,
    /// 감사전재무제표미제출신고서
    F005,
    /// 증권신고집합투자증권신탁형
    // endregion

    // region G 펀드공시
    G001,
    /// 증권신고집합투자증권회사형
    G002,
    /// 증권신고집합투자증권합병
    G003,
    /// 자산유동화계획양도등록
    // endregion

    // region H 자산유동화
    H001,
    /// 사업반기분기보고서
    H002,
    /// 증권신고유동화증권등
    H003,
    /// 채권유동화계획양도등록
    H004,
    /// 자산유동화관련중요사항발생등보고
    H005,
    /// 주요사항보고서
    H006,
    /// 수시공시
    // endregion

    // region I 거래소공시
    I001,
    /// 공정공시
    I002,
    /// 시장조치안내
    I003,
    /// 지분공시
    I004,
    /// 증권투자회사
    I005,
    /// 채권공시
    I006,
    /// 대규모내부거래관련
    // endregion

    // region J 공정위공시
    J001,
    /// 대규모내부거래관련구
    J002,
    /// 기업집단현황공시
    J004,
    /// 비상장회사중요사항공시
    J005,
    /// 기타공정위공시
    J006,
    /// 대규모내부거래관련공익법인용
    J008,
    /// 하도급대금결제조건공시
    J009,
    // endregion
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Context;

    #[test]
    fn serialize() -> anyhow::Result<()> {
        let pblntf_detail_ty = PblntfDetailTy(Inner::F001);
        let serialized = serde_json::to_string(&pblntf_detail_ty).context("Failed to serialize")?;
        assert_eq!(serialized, r#""F001""#);
        Ok(())
    }

    #[test]
    fn deserialize() -> anyhow::Result<()> {
        let pblntf_detail_ty = PblntfDetailTy(Inner::F001);
        let deserialized: PblntfDetailTy =
            serde_json::from_str(r#""F001""#).context("Failed to deserialize")?;
        assert_eq!(deserialized, pblntf_detail_ty);
        Ok(())
    }

    #[test]
    fn display() {
        assert_eq!(PblntfDetailTy(Inner::F001).to_string(), "F001");
    }
}
