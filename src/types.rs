use derive_more::From;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError, ValidationErrors};

// region: CtrfcKey
/// ### API 인증키
/// 발급받은 인증키(40자리)
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CrtfcKey(String);

impl Validate for CrtfcKey {
    fn validate(&self) -> Result<(), ValidationErrors> {
        if self.0.len() != 40 {
            let mut errors = ValidationErrors::new();
            errors.add(
                "crtfc_key",
                ValidationError::new("crtfc_key must be 40 characters"),
            );
            return Err(errors);
        }
        Ok(())
    }
}

impl From<String> for CrtfcKey {
    fn from(s: String) -> Self {
        CrtfcKey(s)
    }
}
// endregion: CtrfcKey

// region: CorpCode
/// ### 고유번호
/// 공시대상회사의 고유번호(8자리)
///
/// ※ 개발가이드 > 공시정보 > 고유번호 참고
#[derive(Clone, Debug, From, Serialize, Deserialize)]
pub struct CorpCode(pub String);

impl Validate for CorpCode {
    fn validate(&self) -> Result<(), ValidationErrors> {
        if self.0.len() != 8 {
            let mut errors = ValidationErrors::new();
            errors.add(
                "corp_code",
                ValidationError::new("corp_code must be 8 characters"),
            );
            return Err(errors);
        }
        Ok(())
    }
}
// endregion: CorpCode

/// ### 법인구분
///
/// - Y : 유가
/// - K : 코스닥
/// - N : 코넥스
/// - E : 기타
///
/// ※ 없으면 전체조회, 복수조건 불가
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CorpCls {
    Y,
    K,
    N,
    E,
    #[serde(untagged)]
    Other(String),
}

/// ### 공시유형
///
/// - A : 정기공시
/// - B : 주요사항보고
/// - C : 발행공시
/// - D : 지분공시
/// - E : 기타공시
/// - F : 외부감사관련
/// - G : 펀드공시
/// - H : 자산유동화
/// - I : 거래소공시
/// - J : 공정위공시
#[allow(dead_code)]
#[derive(Clone, Debug, Serialize)]
pub enum PblntfTy {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    #[serde(untagged)]
    Other(String),
}

/// ### 공시상세유형
///
/// #### A: 정기공시
/// - A001 : 사업보고서
/// - A002 : 반기보고서
/// - A003 : 분기보고서
/// - A004 : 등록법인결산서류(자본시장법이전)
/// - A005 : 소액공모법인결산서류
///
/// #### B: 주요사항보고
/// - B001 : 주요사항보고서
/// - B002 : 주요경영사항신고(자본시장법 이전)
/// - B003 : 최대주주등과의거래신고(자본시장법 이전)
///
/// #### C: 발행공시
/// - C001 : 증권신고(지분증권)
/// - C002 : 증권신고(채무증권)
/// - C003 : 증권신고(파생결합증권)
/// - C004 : 증권신고(합병등)
/// - C005 : 증권신고(기타)
/// - C006 : 소액공모(지분증권)
/// - C007 : 소액공모(채무증권)
/// - C008 : 소액공모(파생결합증권)
/// - C009 : 소액공모(합병등)
/// - C010 : 소액공모(기타)
/// - C011 : 호가중개시스템을통한소액매출
///
/// #### D: 지분공시
/// - D001 : 주식등의대량보유상황보고서
/// - D002 : 임원ㆍ주요주주특정증권등소유상황보고서
/// - D003 : 의결권대리행사권유
/// - D004 : 공개매수
/// - D005 : 임원ㆍ주요주주 특정증권등 거래계획보고서
///
/// #### E: 기타공시
/// - E001 : 자기주식취득/처분
/// - E002 : 신탁계약체결/해지
/// - E003 : 합병등종료보고서
/// - E004 : 주식매수선택권부여에관한신고
/// - E005 : 사외이사에관한신고
/// - E006 : 주주총회소집보고서
/// - E007 : 시장조성/안정조작
/// - E008 : 합병등신고서(자본시장법 이전)
/// - E009 : 금융위등록/취소(자본시장법 이전)
///
/// #### F: 외부감사관련
/// - F001 : 감사보고서
/// - F002 : 연결감사보고서
/// - F003 : 결합감사보고서
/// - F004 : 회계법인사업보고서
/// - F005 : 감사전재무제표미제출신고서
///
/// #### G: 펀드공시
/// - G001 : 증권신고(집합투자증권-신탁형)
/// - G002 : 증권신고(집합투자증권-회사형)
/// - G003 : 증권신고(집합투자증권-합병)
///
/// #### H: 자산유동화
/// - H001 : 자산유동화계획/양도등록
/// - H002 : 사업/반기/분기보고서
/// - H003 : 증권신고(유동화증권등)
/// - H004 : 채권유동화계획/양도등록
/// - H005 : 자산유동화관련중요사항발생등보고
/// - H006 : 주요사항보고서
///
/// #### I: 거래소공시
/// - I001 : 수시공시
/// - I002 : 공정공시
/// - I003 : 시장조치/안내
/// - I004 : 지분공시
/// - I005 : 증권투자회사
/// - I006 : 채권공시
///
/// #### J: 공정위공시
/// - J001 : 대규모내부거래관련
/// - J002 : 대규모내부거래관련(구)
/// - J004 : 기업집단현황공시
/// - J005 : 비상장회사중요사항공시
/// - J006 : 기타공정위공시
/// - J008 : 대규모내부거래관련(공익법인용)
/// - J009 : 하도급대금결제조건공시
#[derive(Clone, Debug, Serialize)]
pub enum PblntfDetailTy {
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
    #[serde(untagged)]
    Other(String),
}
