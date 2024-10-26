//! ## 다중회사 주요계정 개발가이드
//! [link](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS003&apiId=2019017)
//! 상장법인(유가증권, 코스닥) 및 주요 비상장법인(사업보고서 제출대상 & IFRS 적용)이 제출한 정기보고서 내에 XBRL재무제표의 주요계정과목(재무상태표, 손익계산서)을 제공합니다. (대상법인 복수조회 복수조회 가능)
use crate::assert_impl_commons_without_default;
use crate::error::OpenDartError;
use crate::types::{
    BsnsYear, CorpCls, CorpCode, CorpName, CrtfcKey, PageCount, PageNo, ReprtCode, StockCode,
    TotalCount, TotalPage,
};

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

// region: Request Params
/// Documentation exists in each field's types
#[derive(Builder, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenDartError"))]
pub struct FnlttMultiAcntRequestParams {
    #[builder(setter(skip))]
    crtfc_key: CrtfcKey,

    pub corp_code: CorpCode,
    pub bsns_year: BsnsYear,
    pub reprt_code: ReprtCode,
}
assert_impl_commons_without_default!(FnlttMultiAcntRequestParams);

impl std::fmt::Display for FnlttMultiAcntRequestParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
// endregion

// region: Response
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
pub struct List {
    page_no: PageNo,
    page_count: PageCount,
    total_count: TotalCount,
    total_page: TotalPage,

    list: Vec<ListCorp>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
struct ListCorp {
    corp_cls: CorpCls,
    corp_name: CorpName,
    corp_code: CorpCode,
    stock_code: StockCode,

    /// ### 보고서명
    /// 공시구분+보고서명+기타정보
    /// - \[기재정정\] : 본 보고서명으로 이미 제출된 보고서의 기재내용이 변경되어 제출된 것임
    /// - \[첨부정정\] : 본 보고서명으로 이미 제출된 보고서의 첨부내용이 변경되어 제출된 것임
    /// - \[첨부추가\] : 본 보고서명으로 이미 제출된 보고서의 첨부서류가 추가되어 제출된 것임
    /// - \[변경등록\] : 본 보고서명으로 이미 제출된 보고서의 유동화계획이 변경되어 제출된 것임
    /// - \[연장결정\] : 본 보고서명으로 이미 제출된 보고서의 신탁계약이 연장되어 제출된 것임
    /// - \[발행조건확정\] : 본 보고서명으로 이미 제출된 보고서의 유가증권 발행조건이 확정되어 제출된 것임
    /// - \[정정명령부과\] : 본 보고서에 대하여 금융감독원이 정정명령을 부과한 것임
    /// - \[정정제출요구\] : 본 보고서에 대하여 금융감독원이 정정제출요구을 부과한 것임
    report_nm: String,

    /// ### 접수번호
    /// 접수번호(14자리)
    ///
    /// ※ 공시뷰어 연결에 이용예시
    /// - PC용 : https://dart.fss.or.kr/dsaf001/main.do?rcpNo=접수번호
    rcept_no: String,

    /// ### 공시 제출인명
    flr_nm: String,

    /// ### 접수일자
    /// 공시 접수일자(YYYYMMDD)
    rcept_dt: String,

    /// ### 비고
    /// 조합된 문자로 각각은 아래와 같은 의미가 있음
    /// - 유 : 본 공시사항은 한국거래소 유가증권시장본부 소관임
    /// - 코 : 본 공시사항은 한국거래소 코스닥시장본부 소관임
    /// - 채 : 본 문서는 한국거래소 채권상장법인 공시사항임
    /// - 넥 : 본 문서는 한국거래소 코넥스시장 소관임
    /// - 공 : 본 공시사항은 공정거래위원회 소관임
    /// - 연 : 본 보고서는 연결부분을 포함한 것임
    /// - 정 : 본 보고서 제출 후 정정신고가 있으니 관련 보고서를 참조하시기 바람
    /// - 철 : 본 보고서는 철회(간주)되었으니 관련 철회신고서(철회간주안내)를 참고하시기 바람
    rm: String,
}

// endregion

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::MockDefault;
    use crate::types::{BsnsYear, ReprtCode};

    #[test]
    fn list_request_params_builder_works_with_all_fields_specified() {
        let corp_code = CorpCode::mock_default();
        let bsns_year = BsnsYear::mock_default();
        let reprt_code = ReprtCode::mock_default();

        let params = FnlttMultiAcntRequestParamsBuilder::default()
            .corp_code(corp_code.clone())
            .bsns_year(bsns_year.clone())
            .reprt_code(reprt_code.clone())
            .expect("FnlttMultiAcntRequestParams should build");

        assert_eq!(params.corp_code, Some(corp_code));
        assert_eq!(params.bsns_year, Some(bsns_year));
        assert_eq!(params.reprt_code, Some(reprt_code));
    }
}
