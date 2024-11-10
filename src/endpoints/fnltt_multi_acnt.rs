//! ## 다중회사 주요계정 개발가이드
//! [link](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS003&apiId=2019017)
//! 상장법인(유가증권, 코스닥) 및 주요 비상장법인(사업보고서 제출대상 & IFRS 적용)이 제출한 정기보고서 내에 XBRL재무제표의 주요계정과목(재무상태표, 손익계산서)을 제공합니다. (대상법인 복수조회 복수조회 가능)

use crate::endpoints::macros::params;
use crate::validate::fields::{bsns_year, corp_code, reprt_code};

params!(
    #[validate(custom(function = "corp_code"))]
    pub corp_code: String,
    #[validate(custom(function = "bsns_year"))]
    pub bsns_year: String,
    #[validate(custom(function = "reprt_code"))]
    pub reprt_code: String,
);
