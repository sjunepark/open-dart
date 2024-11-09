//! # 공시검색
//! <https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS001&apiId=2019001>
//!
//! 공시 유형별, 회사별, 날짜별 등 여러가지 조건으로 공시보고서 검색기능을 제공합니다.

use crate::client::OpenDartApi;
use crate::endpoints::base::ResponseBody;
use crate::endpoints::macros::{derive_common, json_body, params};
use crate::endpoints::OpenDartResponse;
use crate::error::OpenDartError;
use crate::types::{
    BgnDe, CorpCls, CorpCode, CorpName, FlrNm, LastReprtAt, PageCount, PageNo, PblntfTy, RceptDt,
    RceptNo, ReportNm, Sort, SortMth, StockCode, TotalCount, TotalPage, RM,
};
use crate::types::{EndDe, PblntfDetailTy};

impl OpenDartApi {
    pub async fn get_list(
        &self,
        args: Params,
    ) -> Result<OpenDartResponse<ResponseBody<List>>, OpenDartError> {
        self.get(self.url("/api/list.json"), args).await
    }
}

params!(
    #[builder(default)]
    pub corp_code: Option<String>,
    #[builder(default)]
    pub bgn_de: Option<BgnDe>,
    #[builder(default)]
    pub end_de: Option<EndDe>,
    #[builder(default)]
    pub last_reprt_at: Option<LastReprtAt>,
    #[builder(default)]
    pub pblntf_ty: Option<PblntfTy>,
    #[builder(default)]
    pub pblntf_detail_ty: Option<PblntfDetailTy>,
    #[builder(default)]
    pub corp_cls: Option<CorpCls>,
    #[builder(default)]
    pub sort: Option<Sort>,
    #[builder(default)]
    pub sort_mth: Option<SortMth>,
    #[builder(default)]
    pub page_no: Option<PageNo>,
    #[builder(default)]
    pub page_count: Option<PageCount>,
);

// region: Response

json_body!(List {
    page_no: Option<PageNo>,
    page_count: Option<PageCount>,
    total_count: Option<TotalCount>,
    total_page: Option<TotalPage>,
    list: Vec<ListCorp>,
});

derive_common!(ListCorp {
    corp_code: CorpCode,
    corp_name: CorpName,
    stock_code: StockCode,
    corp_cls: CorpCls,
    report_nm: ReportNm,
    rcept_no: RceptNo,
    flr_nm: FlrNm,
    rcept_dt: RceptDt,
    rm: RM,
});

// endregion: Response

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::tracing::subscribe_tracing_with_span;
    use crate::test_utils::{mock, test_context, MockDefault};
    use crate::types::YesNo;
    use goldrust::Content;

    #[test]
    fn params_builder_works_with_all_fields_specified() {
        let corp_code = mock::corp_code();
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
            .page_no(page_no.clone())
            .page_count(page_count.clone())
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

    #[tokio::test]
    #[tracing::instrument]
    async fn get_list_default() {
        subscribe_tracing_with_span!("test");
        let mut ctx = test_context!("json").await;

        ctx.arrange_test_endpoint_json::<List>("/api/list.json")
            .await;

        // region: Action
        let params = ParamsBuilder::default()
            .corp_code(mock::corp_code())
            .bgn_de(BgnDe::mock_default())
            .build()
            .expect("Failed to build ListRequestParams");
        tracing::debug!(?params, "Request parameters");

        let response = ctx
            .api
            .get_list(params)
            .await
            .expect("get_list should succeed");
        tracing::info!(?response, "Got response");
        // endregion

        // region: Assert
        assert!(
            response.status().is_success(),
            "Response didn't return a status code of 2xx"
        );
        // endregion

        // region: Save response body
        ctx.goldrust
            .save(Content::Json(
                serde_json::to_value(response.body)
                    .expect("Failed to convert to serde_json::Value"),
            ))
            .expect("Failed to save response body");
        // endregion
    }
}
