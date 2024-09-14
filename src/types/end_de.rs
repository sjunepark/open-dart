use crate::types::BgnDe;

/// ### 종료일
/// 검색종료 접수일자(YYYYMMDD)
///
/// - 기본값 : 당일
/// - 고유번호(corp_code)가 없는 경우 검색기간은 3개월로 제한
pub type EndDe = BgnDe;
