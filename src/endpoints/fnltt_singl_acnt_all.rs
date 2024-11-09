//! ## 단일회사 전체 재무제표 개발가이드
//! [link](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS003&apiId=2019020)
//! 상장법인(유가증권, 코스닥) 및 주요 비상장법인(사업보고서 제출대상 & IFRS 적용)이 제출한 정기보고서 내에 XBRL재무제표의 모든계정과목을 제공합니다.

use crate::client::OpenDartApi;
use crate::endpoints::base::ResponseBody;
use crate::endpoints::macros::{derive_common, json_body, params};
use crate::endpoints::OpenDartResponse;
use crate::validate::fields::{bsns_year, corp_code, fs_div, reprt_code};
use crate::OpenDartError;

impl OpenDartApi {
    pub async fn get_fnltt_singl_acnt_all(
        &self,
        args: Params,
    ) -> Result<OpenDartResponse<ResponseBody<FnlttSinglAcntAll>>, OpenDartError> {
        self.get(self.url("/api/fnlttSinglAcntAll.json"), args)
            .await
    }
}

params!(
    #[validate(custom(function = "corp_code"))]
    pub corp_code: String,
    #[validate(custom(function = "bsns_year"))]
    pub bsns_year: String,
    #[validate(custom(function = "reprt_code"))]
    pub reprt_code: String,
    #[validate(custom(function = "fs_div"))]
    pub fs_div: String,
);

json_body!(FnlttSinglAcntAll {
    list: Vec<FnlttSinglAcntAllElement>,
});

derive_common! {
    FnlttSinglAcntAllElement {
        rcept_no: String,
        reprt_code:String,
        bsns_year:String,
        corp_code:String,
        sj_div:String,
        sj_nm:String,
        account_id:String,
        account_nm:String,
        account_detail:String,
        thstrm_nm:String,
        thstrm_amount:String,
        thstrm_add_amount:Option<String>,
        frmtrm_nm:String,
        frmtrm_amount:String,
        frmtrm_q_amount:Option<String>,
        frmtrm_add_amount:Option<String>,
        bfefrmtrm_nm:Option<String>,
        bfefrmtrm_amount:Option<String>,
        ord:String,
        currency:String,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::mock;
    use crate::test_utils::tracing::subscribe_tracing_with_span;
    use goldrust::Content;

    #[tokio::test]
    async fn get_fnltt_singl_acnt_all() {
        subscribe_tracing_with_span!("tests");
        let mut ctx = crate::test_utils::test_context!("json").await;

        ctx.arrange_test_endpoint_json::<ResponseBody<FnlttSinglAcntAll>>(
            "/api/fnlttSinglAcntAll.json",
        )
        .await;

        let params = ParamsBuilder::default()
            .corp_code(mock::corp_code())
            .bsns_year(mock::bsns_year())
            .reprt_code(mock::reprt_code())
            .fs_div(mock::fs_div())
            .build()
            .expect("Failed to build FnlttSinglAcntAllRequestParams");
        tracing::debug!(?params, "Request parameters");

        let response = ctx
            .api
            .get_fnltt_singl_acnt_all(params)
            .await
            .expect("get_fnltt_singl_acnt_all should succeed");

        assert!(
            response.status().is_success(),
            "Response didn't return a status code of 2xx"
        );

        ctx.goldrust
            .save(Content::Json(
                serde_json::to_value(response.body)
                    .expect("Failed to convert to serde_json::Value"),
            ))
            .expect("Failed to save response body");
    }
}
