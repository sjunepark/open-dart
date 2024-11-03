//! # 기업개황
//! <https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS001&apiId=2019002>
//!
//! DART에 등록되어있는 기업의 개황정보를 제공합니다.

use crate::client::OpenDartApi;
use crate::endpoints::base::ResponseBody;
use crate::endpoints::macros::{json_body, params};
use crate::endpoints::OpenDartResponse;
use crate::error::OpenDartError;
use crate::types::{
    AccMt, Adres, BizrNo, CeoNm, CorpName, CorpNameEng, EstDt, FaxNo, HmUrl, IndutyCode, IrUrl,
    JurirNo, PhnNo, StockName,
};
use crate::types::{CorpCls, CorpCode, StockCode};

impl OpenDartApi {
    pub async fn get_company(
        &self,
        args: Params,
    ) -> Result<OpenDartResponse<ResponseBody<Company>>, OpenDartError> {
        self.get(self.url("/api/company.json"), args).await
    }
}

params!(
    pub corp_code: CorpCode,
);

json_body!(Company {
    corp_code: CorpCode,
    corp_name: CorpName,
    corp_name_eng: CorpNameEng,
    stock_name: StockName,
    stock_code: StockCode,
    ceo_nm: CeoNm,
    corp_cls: CorpCls,
    jurir_no: JurirNo,
    bizr_no: BizrNo,
    adres: Adres,
    hm_url: HmUrl,
    ir_url: IrUrl,
    phn_no: PhnNo,
    fax_no: FaxNo,
    induty_code: IndutyCode,
    est_dt: EstDt,
    acc_mt: AccMt,
});

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::MessageError;
    use crate::test_utils::tracing_setup::subscribe_tracing_with_span;
    use crate::test_utils::{test_context, MockDefault};
    use goldrust::Content;

    #[test]
    fn params_builder_works_with_all_fields_specified() {
        let corp_code = CorpCode::mock_default();

        let params = ParamsBuilder::default()
            .corp_code(corp_code.clone())
            .build()
            .expect("Failed to build Params");

        assert_eq!(params.corp_code, corp_code);
    }

    #[tokio::test]
    #[tracing::instrument]
    async fn get_company_default() {
        subscribe_tracing_with_span!("test");
        let mut ctx = test_context!("json").await;

        ctx.arrange_test_endpoint_json::<ResponseBody<Company>>("/api/company.json")
            .await;

        // region: Action
        let params = ParamsBuilder::default()
            .corp_code(CorpCode::mock_default())
            .build()
            .expect("Failed to build CompanyRequestParams");
        tracing::debug!(?params, "Request parameters");

        let response = ctx
            .api
            .get_company(params)
            .await
            .expect("get_company should succeed");
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

    #[tokio::test]
    async fn get_should_fail_when_receiving_dart_message_code_other_than_000() {
        // Set an invalid API key to get a 200 response with an error message
        std::env::set_var(
            "OPEN_DART_API_KEY",
            "0123456789012345678901234567890123456789",
        );

        subscribe_tracing_with_span!("test");
        let mut ctx = test_context!("json").await;

        ctx.arrange_test_endpoint_json::<ResponseBody<Company>>("/api/company.json")
            .await;

        // region: Action
        let params = ParamsBuilder::default()
            .corp_code(CorpCode::mock_default())
            .build()
            .expect("Failed to build CompanyRequestParams");
        tracing::debug!(?params, "Request parameters");

        let error = ctx
            .api
            .get_company(params)
            .await
            .expect_err("get_company should fail");
        // endregion

        // region: Assert
        assert!(matches!(error, OpenDartError::Message(MessageError { .. })));
        // endregion

        // region: Save response body
        if let OpenDartError::Message(error) = error {
            ctx.goldrust
                .save(Content::Json(
                    serde_json::to_value(error).expect("Failed to convert to serde_json::Value"),
                ))
                .expect("Failed to save response body");
        }
        // endregion
    }
}
