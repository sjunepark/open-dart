//! # 공시검색
//! <https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS001&apiId=2019001>
//!
//! 공시 유형별, 회사별, 날짜별 등 여러가지 조건으로 공시보고서 검색기능을 제공합니다.

use crate::client::OpenDartApi;
use crate::endpoints::base::ResponseBody;
use crate::endpoints::macros::{derive_common, json_body, params};
use crate::endpoints::OpenDartResponse;
use crate::error::OpenDartError;
use crate::validate::fields::*;

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
    #[validate(custom(function = "optional_corp_code"))]
    pub corp_code: Option<String>,
    #[builder(default)]
    #[validate(custom(function = "optional_yyyymmdd"))]
    pub bgn_de: Option<String>,
    #[builder(default)]
    #[validate(custom(function = "optional_yyyymmdd"))]
    pub end_de: Option<String>,
    #[builder(default)]
    #[validate(custom(function = "optional_yes_no"))]
    pub last_reprt_at: Option<String>,
    #[builder(default)]
    #[validate(custom(function = "optional_pblntf_ty"))]
    pub pblntf_ty: Option<String>,
    #[builder(default)]
    #[validate(custom(function = "optional_pblntf_detail_ty"))]
    pub pblntf_detail_ty: Option<String>,
    #[builder(default)]
    #[validate(custom(function = "optional_corp_cls"))]
    pub corp_cls: Option<String>,
    #[builder(default)]
    #[validate(custom(function = "optional_sort"))]
    pub sort: Option<String>,
    #[builder(default)]
    #[validate(custom(function = "optional_sort_mth"))]
    pub sort_mth: Option<String>,
    #[builder(default)]
    #[validate(range(min = 1))]
    pub page_no: Option<u64>,
    #[builder(default)]
    #[validate(range(min = 1))]
    pub page_count: Option<u64>,
);

// region: Response

json_body!(List {
    page_no: Option<u64>,
    page_count: Option<u64>,
    total_count: Option<u64>,
    total_page: Option<u64>,
    list: Vec<ListElement>,
});

derive_common!(ListElement {
    corp_code: String,
    corp_name: String,
    stock_code: String,
    corp_cls: String,
    report_nm: String,
    rcept_no: String,
    flr_nm: String,
    rcept_dt: String,
    rm: String,
});

// endregion: Response

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::tracing::subscribe_tracing_with_span;
    use crate::test_utils::{mock, test_context};
    use goldrust::Content;

    #[test]
    fn params_builder_works_with_all_fields_specified() {
        let corp_code = mock::corp_code();
        let bgn_de = mock::yyyymmdd();
        let end_de = mock::yyyymmdd();
        let last_reprt_at = mock::yes_no();
        let pblntf_ty = mock::pblntf_ty();
        let pblntf_detail_ty = mock::pbntf_detail_ty();
        let corp_cls = mock::corp_cls();
        let sort = mock::sort();
        let sort_mth = mock::sort_mth();
        let page_no = 1;
        let page_count = 1;

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
            .bgn_de(mock::yyyymmdd())
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
