use self::Inner::*;
use crate::assert_impl_commons_without_default;
use derive_more::{AsMut, AsRef, Display};
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

impl PblntfDetailTy {
    // region: A: 정기공시
    /// A001 : 사업보고서
    pub const A001: Self = Self(A001);
    /// A002 : 반기보고서
    pub const A002: Self = Self(A002);
    /// A003 : 분기보고서
    pub const A003: Self = Self(A003);
    /// A004 : 등록법인결산서류(자본시장법이전)
    pub const A004: Self = Self(A004);
    /// A005 : 소액공모법인결산서류
    pub const A005: Self = Self(A005);
    // endregion: A: 정기공시

    // region: B: 주요사항보고
    /// B001 : 주요사항보고서
    pub const B001: Self = Self(B001);
    /// B002 : 주요경영사항신고(자본시장법 이전)
    pub const B002: Self = Self(B002);
    /// B003 : 최대주주등과의거래신고(자본시장법 이전)
    pub const B003: Self = Self(B003);
    // endregion: B: 주요사항보고

    // region: C: 발행공시
    /// C001 : 증권신고(지분증권)
    pub const C001: Self = Self(C001);
    /// C002 : 증권신고(채무증권)
    pub const C002: Self = Self(C002);
    /// C003 : 증권신고(파생결합증권)
    pub const C003: Self = Self(C003);
    /// C004 : 증권신고(합병등)
    pub const C004: Self = Self(C004);
    /// C005 : 증권신고(기타)
    pub const C005: Self = Self(C005);
    /// C006 : 소액공모(지분증권)
    pub const C006: Self = Self(C006);
    /// C007 : 소액공모(채무증권)
    pub const C007: Self = Self(C007);
    /// C008 : 소액공모(파생결합증권)
    pub const C008: Self = Self(C008);
    /// C009 : 소액공모(합병등)
    pub const C009: Self = Self(C009);
    /// C010 : 소액공모(기타)
    pub const C010: Self = Self(C010);
    /// C011 : 호가중개시스템을통한소액매출
    pub const C011: Self = Self(C011);
    // endregion: C: 발행공시

    // region: D: 지분공시
    /// D001 : 주식등의대량보유상황보고서
    pub const D001: Self = Self(D001);
    /// D002 : 임원ㆍ주요주주특정증권등소유상황보고서
    pub const D002: Self = Self(D002);
    /// D003 : 의결권대리행사권유
    pub const D003: Self = Self(D003);
    /// D004 : 공개매수
    pub const D004: Self = Self(D004);
    /// D005 : 임원ㆍ주요주주 특정증권등 거래계획보고서
    pub const D005: Self = Self(D005);
    // endregion: D: 지분공시

    // region: E: 기타공시
    /// E001 : 자기주식취득/처분
    pub const E001: Self = Self(E001);
    /// E002 : 신탁계약체결/해지
    pub const E002: Self = Self(E002);
    /// E003 : 합병등종료보고서
    pub const E003: Self = Self(E003);
    /// E004 : 주식매수선택권부여에관한신고
    pub const E004: Self = Self(E004);
    /// E005 : 사외이사에관한신고
    pub const E005: Self = Self(E005);
    /// E006 : 주주총회소집보고서
    pub const E006: Self = Self(E006);
    /// E007 : 시장조성/안정조작
    pub const E007: Self = Self(E007);
    /// E008 : 합병등신고서(자본시장법 이전)
    pub const E008: Self = Self(E008);
    /// E009 : 금융위등록/취소(자본시장법 이전)
    pub const E009: Self = Self(E009);
    // endregion: E: 기타공시

    // region: F: 외부감사관련
    /// F001 : 감사보고서
    pub const F001: Self = Self(F001);
    /// F002 : 연결감사보고서
    pub const F002: Self = Self(F002);
    /// F003 : 결합감사보고서
    pub const F003: Self = Self(F003);
    /// F004 : 회계법인사업보고서
    pub const F004: Self = Self(F004);
    /// F005 : 감사전재무제표미제출신고서
    pub const F005: Self = Self(F005);
    // endregion: F: 외부감사관련

    // region: G: 펀드공시
    /// G001 : 증권신고(집합투자증권-신탁형)
    pub const G001: Self = Self(G001);
    /// G002 : 증권신고(집합투자증권-회사형)
    pub const G002: Self = Self(G002);
    /// G003 : 증권신고(집합투자증권-합병)
    pub const G003: Self = Self(G003);
    // endregion: G: 펀드공시

    // region: H: 자산유동화
    /// H001 : 자산유동화계획/양도등록
    pub const H001: Self = Self(H001);
    /// H002 : 사업/반기/분기보고서
    pub const H002: Self = Self(H002);
    /// H003 : 증권신고(유동화증권등)
    pub const H003: Self = Self(H003);
    /// H004 : 채권유동화계획/양도등록
    pub const H004: Self = Self(H004);
    /// H005 : 자산유동화관련중요사항발생등보고
    pub const H005: Self = Self(H005);
    /// H006 : 주요사항보고서
    pub const H006: Self = Self(H006);
    // endregion: H: 자산유동화

    // region: I: 거래소공시
    /// I001 : 수시공시
    pub const I001: Self = Self(I001);
    /// I002 : 공정공시
    pub const I002: Self = Self(I002);
    /// I003 : 시장조치/안내
    pub const I003: Self = Self(I003);
    /// I004 : 지분공시
    pub const I004: Self = Self(I004);
    /// I005 : 증권투자회사
    pub const I005: Self = Self(I005);
    /// I006 : 채권공시
    pub const I006: Self = Self(I006);
    // endregion: I: 거래소공시

    // region: J: 공정위공시
    /// J001 : 대규모내부거래관련
    pub const J001: Self = Self(J001);
    /// J002 : 대규모내부거래관련(구)
    pub const J002: Self = Self(J002);
    /// J004 : 기업집단현황공시
    pub const J004: Self = Self(J004);
    /// J005 : 비상장회사중요사항공시
    pub const J005: Self = Self(J005);
    /// J006 : 기타공정위공시
    pub const J006: Self = Self(J006);
    /// J008 : 대규모내부거래관련(공익법인용)
    pub const J008: Self = Self(J008);
    /// J009 : 하도급대금결제조건공시
    pub const J009: Self = Self(J009);
    // endregion: J: 공정위공시
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Display, Serialize, Deserialize)]
#[display("{_variant}")]
enum Inner {
    A001,
    A002,
    A003,
    A004,
    A005,
    B001,
    B002,
    B003,
    C001,
    C002,
    C003,
    C004,
    C005,
    C006,
    C007,
    C008,
    C009,
    C010,
    C011,
    D001,
    D002,
    D003,
    D004,
    D005,
    E001,
    E002,
    E003,
    E004,
    E005,
    E006,
    E007,
    E008,
    E009,
    F001,
    F002,
    F003,
    F004,
    F005,
    G001,
    G002,
    G003,
    H001,
    H002,
    H003,
    H004,
    H005,
    H006,
    I001,
    I002,
    I003,
    I004,
    I005,
    I006,
    J001,
    J002,
    J004,
    J005,
    J006,
    J008,
    J009,
}

#[cfg(test)]
mod tests {}
