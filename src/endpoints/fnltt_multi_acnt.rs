//! ## 다중회사 주요계정 개발가이드
//! [link](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS003&apiId=2019017)
//! 상장법인(유가증권, 코스닥) 및 주요 비상장법인(사업보고서 제출대상 & IFRS 적용)이 제출한 정기보고서 내에 XBRL재무제표의 주요계정과목(재무상태표, 손익계산서)을 제공합니다. (대상법인 복수조회 복수조회 가능)
use crate::assert_impl_commons_without_default;
use crate::error::OpenDartError;
use crate::types::{BsnsYear, CorpCode, CrtfcKey, ReprtCode};

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
