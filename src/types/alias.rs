use crate::types::text::Text;
use crate::types::{Date, PageNo, YesNo};

/// ## 시작일
/// 검색시작 접수일자(YYYYMMDD)
///
/// - 기본값 : 종료일(end_de)
/// - 고유번호(corp_code)가 없는 경우 검색기간은 3개월로 제한
pub type BgnDe = Date;

/// ## 종료일
/// 검색종료 접수일자(YYYYMMDD)
///
/// - 기본값 : 당일
/// - 고유번호(corp_code)가 없는 경우 검색기간은 3개월로 제한
pub type EndDe = Date;

/// ## 총 건수
/// 총 페이지 수
pub type TotalCount = PageNo;

/// ## 총 페이지 수
pub type TotalPage = PageNo;

/// ## 최종보고서 검색여부
/// 최종보고서만 검색여부(Y or N)
///
/// - 기본값 : N(정정이 있는 경우 최종정정만 검색)
pub type LastReprtAt = YesNo;

/// ## 종목명(법인명)
/// 공시대상회사의 종목명(상장사) 또는 법인명(기타법인)
pub type CorpName = Text;

/// ## 영문정식회사명칭
pub type CorpNameEng = Text;

/// ## 보고서명
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
pub type ReportNm = Text;

/// ## 종목명(상장사) 또는 약식명칭(기타법인)
pub type StockName = Text;

/// ## 대표자명
pub type CeoNm = Text;

/// ## 주소
pub type Adres = Text;

/// ## 홈페이지
pub type HmUrl = Text;

/// ## IR홈페이지
pub type IrUrl = Text;

/// ## 전화번호
pub type PhnNo = Text;

/// ## 팩스번호
pub type FaxNo = Text;

/// ## 설립일
///
/// YYYYMMDD
pub type EstDt = Date;
