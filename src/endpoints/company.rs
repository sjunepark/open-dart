//! # 기업개황
//! <https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS001&apiId=2019002>
//!
//! DART에 등록되어있는 기업의 개황정보를 제공합니다.

use crate::client::OpenDartApi;
use crate::endpoints::base::Message;
use crate::endpoints::OpenDartResponse;
use crate::error::OpenDartError;
use crate::statics::assert_impl_commons_without_default;
use crate::types::{
    AccMt, Adres, BizrNo, CeoNm, CorpName, EstDt, FaxNo, HmUrl, IndutyCode, IrUrl, JurirNo, PhnNo,
    StockName,
};
use crate::types::{CorpCls, CorpCode, CrtfcKey, StockCode};
use derive_builder::Builder;
use derive_more::{Display, From, Into};
use serde::{Deserialize, Serialize};

impl OpenDartApi {
    pub async fn get_company(
        &self,
        args: Params,
    ) -> Result<OpenDartResponse<ResponseBody>, OpenDartError> {
        self.get(self.url("/api/company.json"), args).await
    }
}

// region: Request Params

assert_impl_commons_without_default!(Params);
/// Documentation exists in each field's types
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
    // builder
    Builder,
)]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenDartError"))]
#[display("{self:?}")]
pub struct Params {
    #[builder(setter(skip))]
    crtfc_key: CrtfcKey,

    pub corp_code: CorpCode,
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
#[display("{self:?}")]
pub struct ResponseBody {
    #[serde(flatten)]
    pub message: Message,

    corp_name: CorpName,
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
}
// endregion: Response

#[cfg(test)]
mod tests {
    use super::*;
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
        let mut ctx = test_context!().await;

        ctx.arrange_test_endpoint::<ResponseBody>("/api/company.json")
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
}
