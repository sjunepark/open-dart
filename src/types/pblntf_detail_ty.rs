use self::Inner::*;
use crate::error::OpenDartError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

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
#[derive(Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
pub struct PblntfDetailTy(Inner);

#[derive(Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
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

impl FromStr for PblntfDetailTy {
    type Err = OpenDartError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A001" => Ok(PblntfDetailTy(A001)),
            "A002" => Ok(PblntfDetailTy(A002)),
            "A003" => Ok(PblntfDetailTy(A003)),
            "A004" => Ok(PblntfDetailTy(A004)),
            "A005" => Ok(PblntfDetailTy(A005)),
            "B001" => Ok(PblntfDetailTy(B001)),
            "B002" => Ok(PblntfDetailTy(B002)),
            "B003" => Ok(PblntfDetailTy(B003)),
            "C001" => Ok(PblntfDetailTy(C001)),
            "C002" => Ok(PblntfDetailTy(C002)),
            "C003" => Ok(PblntfDetailTy(C003)),
            "C004" => Ok(PblntfDetailTy(C004)),
            "C005" => Ok(PblntfDetailTy(C005)),
            "C006" => Ok(PblntfDetailTy(C006)),
            "C007" => Ok(PblntfDetailTy(C007)),
            "C008" => Ok(PblntfDetailTy(C008)),
            "C009" => Ok(PblntfDetailTy(C009)),
            "C010" => Ok(PblntfDetailTy(C010)),
            "C011" => Ok(PblntfDetailTy(C011)),
            "D001" => Ok(PblntfDetailTy(D001)),
            "D002" => Ok(PblntfDetailTy(D002)),
            "D003" => Ok(PblntfDetailTy(D003)),
            "D004" => Ok(PblntfDetailTy(D004)),
            "D005" => Ok(PblntfDetailTy(D005)),
            "E001" => Ok(PblntfDetailTy(E001)),
            "E002" => Ok(PblntfDetailTy(E002)),
            "E003" => Ok(PblntfDetailTy(E003)),
            "E004" => Ok(PblntfDetailTy(E004)),
            "E005" => Ok(PblntfDetailTy(E005)),
            "E006" => Ok(PblntfDetailTy(E006)),
            "E007" => Ok(PblntfDetailTy(E007)),
            "E008" => Ok(PblntfDetailTy(E008)),
            "E009" => Ok(PblntfDetailTy(E009)),
            "F001" => Ok(PblntfDetailTy(F001)),
            "F002" => Ok(PblntfDetailTy(F002)),
            "F003" => Ok(PblntfDetailTy(F003)),
            "F004" => Ok(PblntfDetailTy(F004)),
            "F005" => Ok(PblntfDetailTy(F005)),
            "G001" => Ok(PblntfDetailTy(G001)),
            "G002" => Ok(PblntfDetailTy(G002)),
            "G003" => Ok(PblntfDetailTy(G003)),
            "H001" => Ok(PblntfDetailTy(H001)),
            "H002" => Ok(PblntfDetailTy(H002)),
            "H003" => Ok(PblntfDetailTy(H003)),
            "H004" => Ok(PblntfDetailTy(H004)),
            "H005" => Ok(PblntfDetailTy(H005)),
            "H006" => Ok(PblntfDetailTy(H006)),
            "I001" => Ok(PblntfDetailTy(I001)),
            "I002" => Ok(PblntfDetailTy(I002)),
            "I003" => Ok(PblntfDetailTy(I003)),
            "I004" => Ok(PblntfDetailTy(I004)),
            "I005" => Ok(PblntfDetailTy(I005)),
            "I006" => Ok(PblntfDetailTy(I006)),
            "J001" => Ok(PblntfDetailTy(J001)),
            "J002" => Ok(PblntfDetailTy(J002)),
            "J004" => Ok(PblntfDetailTy(J004)),
            "J005" => Ok(PblntfDetailTy(J005)),
            "J006" => Ok(PblntfDetailTy(J006)),
            "J008" => Ok(PblntfDetailTy(J008)),
            "J009" => Ok(PblntfDetailTy(J009)),
            _ => Err(OpenDartError::InvalidArgument(s.to_string())),
        }
    }
}
