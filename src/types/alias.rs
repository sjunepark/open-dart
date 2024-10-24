use crate::types::{Date, PageNo, YesNo};

/// ### 시작일
/// 검색시작 접수일자(YYYYMMDD)
///
/// - 기본값 : 종료일(end_de)
/// - 고유번호(corp_code)가 없는 경우 검색기간은 3개월로 제한
pub type BgnDe = Date;

/// ### 종료일
/// 검색종료 접수일자(YYYYMMDD)
///
/// - 기본값 : 당일
/// - 고유번호(corp_code)가 없는 경우 검색기간은 3개월로 제한
pub type EndDe = Date;

/// ### 총 건수
/// 총 페이지 수
pub type TotalCount = PageNo;

/// ### 총 페이지 수
pub type TotalPage = PageNo;

/// ### 최종보고서 검색여부
/// 최종보고서만 검색여부(Y or N)
///
/// - 기본값 : N(정정이 있는 경우 최종정정만 검색)
pub type LastReprtAt = YesNo;
