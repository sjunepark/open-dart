//! ## 공시검색
//! [link](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS001&apiId=2019001)
//! 공시 유형별, 회사별, 날짜별 등 여러가지 조건으로 공시보고서 검색기능을 제공합니다.
use crate::assert_impl_commons;
use crate::error::OpenDartError;
use crate::types::{
    BgnDe, CorpCls, CorpCode, CorpName, CrtfcKey, PageCount, PageNo, PblntfTy, Sort, SortMth,
    StockCode, TotalCount, TotalPage, YesNo,
};
use crate::types::{EndDe, PblntfDetailTy};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

assert_impl_commons!(ListRequestParams);

// region: Request Params
/// Documentation exists in each field's types
#[derive(
    Builder, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize, Default,
)]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenDartError"))]
pub struct ListRequestParams {
    #[builder(setter(skip))]
    crtfc_key: CrtfcKey,

    pub corp_code: Option<CorpCode>,
    pub bgn_de: Option<BgnDe>,
    pub end_de: Option<EndDe>,
    pub last_reprt_at: Option<YesNo>,
    pub pblntf_ty: Option<PblntfTy>,
    pub pblntf_detail_ty: Option<PblntfDetailTy>,
    pub corp_cls: Option<CorpCls>,
    pub sort: Option<Sort>,
    pub sort_mth: Option<SortMth>,
    pub page_no: Option<PageNo>,
    pub page_count: Option<PageCount>,
}

impl std::fmt::Display for ListRequestParams {
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
    use anyhow::Context;

    #[test]
    fn list_request_params_builder_works_with_all_fields_specified() -> anyhow::Result<()> {
        let corp_code = CorpCode::mock_default();
        let bgn_de = BgnDe::mock_default();
        let end_de = EndDe::mock_default();
        let last_reprt_at = YesNo::mock_default();
        let pblntf_ty = PblntfTy::mock_default();
        let pblntf_detail_ty = PblntfDetailTy::mock_default();
        let corp_cls = CorpCls::mock_default();
        let sort = Sort::mock_default();
        let sort_mth = SortMth::mock_default();
        let page_no = PageNo::mock_default();
        let page_count = PageCount::mock_default();

        let params = ListRequestParamsBuilder::default()
            .corp_code(corp_code.clone())
            .bgn_de(bgn_de.clone())
            .end_de(end_de.clone())
            .last_reprt_at(last_reprt_at.clone())
            .pblntf_ty(pblntf_ty.clone())
            .pblntf_detail_ty(pblntf_detail_ty.clone())
            .corp_cls(corp_cls.clone())
            .sort(sort.clone())
            .sort_mth(sort_mth.clone())
            .page_no(page_no)
            .page_count(page_count)
            .build()
            .context("ListRequestParams should build")?;

        assert_eq!(params.corp_code, Some(corp_code));
        assert_eq!(params.bgn_de, Some(bgn_de));
        assert_eq!(params.end_de, Some(end_de));
        assert_eq!(params.last_reprt_at, Some(last_reprt_at));
        assert_eq!(params.pblntf_ty, Some(pblntf_ty));
        assert_eq!(params.pblntf_detail_ty, Some(pblntf_detail_ty));
        assert_eq!(params.corp_cls, Some(corp_cls));
        assert_eq!(params.sort, Some(sort));
        assert_eq!(params.sort_mth, Some(sort_mth));
        assert_eq!(params.page_no, Some(page_no));
        assert_eq!(params.page_count, Some(page_count));

        Ok(())
    }
}
