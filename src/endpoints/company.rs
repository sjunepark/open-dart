//! # 기업개황
//! <https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS001&apiId=2019002>
//!
//! DART에 등록되어있는 기업의 개황정보를 제공합니다.

use crate::endpoints::base::Message;
use crate::error::OpenDartError;
use crate::statics::{assert_impl_commons, assert_impl_commons_without_default};
use crate::types::{
    Adres, BizrNo, CeoNm, CorpName, EstDt, FaxNo, HmUrl, IndustryCode, IrUrl, JurirNo, PhnNo,
    StockName,
};
use crate::types::{CorpCls, CorpCode, CrtfcKey, StockCode};
use derive_builder::Builder;
use derive_more::{Display, From, Into};
use serde::{Deserialize, Serialize};

// region: Request Params
assert_impl_commons!(Params);
/// Documentation exists in each field's types
#[derive(
    Debug,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Default,
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
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenDartError"))]
#[display("{self:?}")]
pub struct Params {
    #[builder(setter(skip))]
    crtfc_key: CrtfcKey,

    pub corp_code: Option<CorpCode>,
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
    industry_code: IndustryCode,
    est_dt: EstDt,
}
// endregion: Response

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::MockDefault;

    #[test]
    fn params_builder_works_with_all_fields_specified() {
        let corp_code = CorpCode::mock_default();

        let params = ParamsBuilder::default()
            .corp_code(corp_code.clone())
            .build()
            .expect("Failed to build Params");

        assert_eq!(params.corp_code, Some(corp_code));
    }
}
