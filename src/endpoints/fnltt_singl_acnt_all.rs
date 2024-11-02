//! ## 단일회사 전체 재무제표 개발가이드
//! [link](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS003&apiId=2019020)
//! 상장법인(유가증권, 코스닥) 및 주요 비상장법인(사업보고서 제출대상 & IFRS 적용)이 제출한 정기보고서 내에 XBRL재무제표의 모든계정과목을 제공합니다.

use crate::endpoints::macros::params;
use crate::types::CorpCode;

params!(
    pub corp_code: CorpCode,
);
