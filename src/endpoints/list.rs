use derive_builder::Builder;
use serde::Serialize;

use crate::env::OpenDartApiKey;
use crate::error::OpenDartError;

/// # 공시검색
/// [link](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS001&apiId=2019001)
/// 공시 유형별, 회사별, 날짜별 등 여러가지 조건으로 공시보고서 검색기능을 제공합니다.
#[derive(Builder, Debug, Default, Serialize)]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenDartError"))]
pub struct ListRequestParams {
    /// ### API 인증키
    /// 발급받은 인증키(40자리)
    #[builder(default = "Self::open_dart_api_key()")]
    #[builder(setter(skip))]
    pub crtfc_key: String,

    /// ### 고유번호
    /// 공시대상회사의 고유번호(8자리)
    ///
    /// ※ 개발가이드 > 공시정보 > 고유번호 참고
    pub corp_code: Option<String>,

    /// ### 시작일
    /// 검색시작 접수일자(YYYYMMDD)
    ///
    /// - 기본값 : 종료일(end_de)
    /// - 고유번호(corp_code)가 없는 경우 검색기간은 3개월로 제한
    pub bgn_de: Option<String>,

    /// ### 종료일
    /// 검색종료 접수일자(YYYYMMDD)
    ///
    /// - 기본값 : 당일
    /// - 고유번호(corp_code)가 없는 경우 검색기간은 3개월로 제한
    pub end_de: Option<String>,

    /// ### 최종보고서 검색여부
    /// 최종보고서만 검색여부(Y or N)
    ///
    /// - 기본값 : N(정정이 있는 경우 최종정정만 검색)
    pub last_reprt_at: Option<String>,

    /// ### 공시유형
    ///
    /// - A : 정기공시
    /// - B : 주요사항보고
    /// - C : 발행공시
    /// - D : 지분공시
    /// - E : 기타공시
    /// - F : 외부감사관련
    /// - G : 펀드공시
    /// - H : 자산유동화
    /// - I : 거래소공시
    /// - J : 공정위공시
    pub pblntf_ty: Option<String>,

    /// ### 공시상세유형
    /// (※ 상세 유형 참조 : pblntf_detail_ty)
    pub pblntf_detail_ty: Option<String>,

    /// ### 법인구분
    ///
    /// - Y : 유가
    /// - K : 코스닥
    /// - N : 코넥스
    /// - E : 기타
    ///
    /// ※ 없으면 전체조회, 복수조건 불가
    pub corp_cls: Option<String>,

    /// ### 정렬
    ///
    /// - date : 접수일자
    /// - crp : 회사명
    /// - rpt : 보고서명
    ///
    /// ※ 기본값 : date
    pub sort: Option<String>,

    /// ### 정렬방법
    ///
    /// - asc : 오름차순
    /// - desc : 내림차순
    ///
    /// ※ 기본값 : desc
    pub sort_mth: Option<String>,

    /// ### 페이지 번호
    /// 페이지 번호(1~n)
    ///
    /// - 기본값 : 1
    pub page_no: Option<String>,

    /// ### 페이지 별 건수
    /// 페이지당 건수(1~100)
    ///
    /// - 기본값 : 10
    /// - 최대값 : 100
    pub page_count: Option<String>,
}

impl OpenDartApiKey for ListRequestParamsBuilder {}
