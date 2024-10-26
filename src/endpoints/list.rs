//! ## 공시검색
//! [link](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS001&apiId=2019001)
//! 공시 유형별, 회사별, 날짜별 등 여러가지 조건으로 공시보고서 검색기능을 제공합니다.

use crate::error::OpenDartError;
use crate::types::{
    BgnDe, CorpCls, CorpCode, CorpName, CrtfcKey, LastReprtAt, PageCount, PageNo, PblntfTy,
    ReportNm, Sort, SortMth, StockCode, TotalCount, TotalPage,
};
use crate::types::{EndDe, PblntfDetailTy};
use crate::{assert_impl_commons, assert_impl_commons_without_default};

use crate::endpoints::base::Message;
use derive_builder::Builder;
use derive_more::{Display, From, Into};
use serde::{Deserialize, Serialize};

// region: Request Params
/// Documentation exists in each field's types
#[derive(
    Builder, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize, Default,
)]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenDartError"))]
pub struct Params {
    #[builder(setter(skip))]
    crtfc_key: CrtfcKey,

    pub corp_code: Option<CorpCode>,
    pub bgn_de: Option<BgnDe>,
    pub end_de: Option<EndDe>,
    pub last_reprt_at: Option<LastReprtAt>,
    pub pblntf_ty: Option<PblntfTy>,
    pub pblntf_detail_ty: Option<PblntfDetailTy>,
    pub corp_cls: Option<CorpCls>,
    pub sort: Option<Sort>,
    pub sort_mth: Option<SortMth>,
    pub page_no: Option<PageNo>,
    pub page_count: Option<PageCount>,
}
assert_impl_commons!(Params);

impl std::fmt::Display for Params {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
// endregion: Request Params

// region: Response
assert_impl_commons_without_default!(ResponseBody);
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
    From,
    Into,
    // serde
    Serialize,
    Deserialize,
)]
#[display("page_no: {page_no}, page_count: {page_count}, total_count: {total_count}, total_page: {total_page}")]
pub struct ResponseBody {
    #[serde(flatten)]
    pub message: Message,

    page_no: PageNo,
    page_count: PageCount,
    total_count: TotalCount,
    total_page: TotalPage,

    list: Vec<ListCorp>,
}

assert_impl_commons_without_default!(ListCorp);
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
    From,
    Into,
    // serde
    Serialize,
    Deserialize,
)]
#[display("corp_code: {corp_code}, corp_name: {corp_name}")]
struct ListCorp {
    corp_code: CorpCode,
    corp_name: CorpName,
    stock_code: StockCode,
    corp_cls: CorpCls,
    report_nm: ReportNm,

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
// endregion: Response

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::MockDefault;
    use crate::types::YesNo;

    #[test]
    fn list_request_params_builder_works_with_all_fields_specified() {
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

        let params = ParamsBuilder::default()
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
            .expect("ListRequestParams should build");

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
    }
}
