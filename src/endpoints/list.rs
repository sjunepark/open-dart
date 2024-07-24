//! ## 공시검색
//! [link](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS001&apiId=2019001)
//! 공시 유형별, 회사별, 날짜별 등 여러가지 조건으로 공시보고서 검색기능을 제공합니다.
use crate::endpoints::Message;
use crate::env::OpenDartApiKey;
use crate::error::OpenDartError;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Deserialize)]
pub struct ListResponse {
    #[serde(flatten)]
    message: Message,

    #[serde(flatten)]
    content: Option<Content>,
}

// todo: find a elegant way to handle this
impl ListResponse {
    pub fn validate(self) -> Result<Self, OpenDartError> {
        self.message.validate()?;
        Ok(self)
    }
}

#[derive(Debug, Deserialize)]
struct Content {
    /// ### 페이지 번호
    pub page_no: i32,

    /// ### 페이지 별 건수
    pub page_count: i32,

    /// ### 총 건수
    /// 총 페이지 수
    pub total_count: i32,

    /// ### 총 페이지 수
    pub total_page: i32,

    pub list: Vec<ListItem>,
}

#[derive(Debug, Deserialize)]
struct ListItem {
    /// ### 법인구분
    /// - Y : 유가
    /// - K : 코스닥
    /// - N : 코넥스
    /// - E : 기타
    pub corp_cls: String,

    /// ### 종목명(법인명)
    /// 공시대상회사의 종목명(상장사) 또는 법인명(기타법인)
    pub corp_name: String,

    /// ### 고유번호
    /// 공시대상회사의 고유번호(8자리)
    pub corp_code: String,

    /// ### 종목코드
    /// 상장회사의 종목코드(6자리)
    pub stock_code: String,

    /// ### 보고서명
    /// 공시구분+보고서명+기타정보
    /// - \[기재정정\] : 본 보고서명으로 이미 제출된 보고서의 기재내용이 변경되어 제출된 것임
    /// - \[첨부정정\] : 본 보고서명으로 이미 제출된 보고서의 첨부내용이 변경되어 제출된 것임
    /// - \[첨부추가\] : 본 보고서명으로 이미 제출된 보고서의 첨부서류가 추가되어 제출된 것임
    /// - \[변경등록\] : 본 보고서명으로 이미 제출된 보고서의 유동화계획이 변경되어 제출된 것임
    /// - \[연장결정\] : 본 보고서명으로 이미 제출된 보고서의 신탁계약이 연장되어 제출된 것임
    /// - \[발행조건확정\] : 본 보고서명으로 이미 제출된 보고서의 유가증권 발행조건이 확정되어 제출된 것임
    /// - \[정정명령부과\] : 본 보고서에 대하여 금융감독원이 정정명령을 부과한 것임
    /// - \[정정제출요구\] : 본 보고서에 대하여 금융감독원이 정정제출요구을 부과한 것임
    pub report_nm: String,

    /// ### 접수번호
    /// 접수번호(14자리)
    ///
    /// ※ 공시뷰어 연결에 이용예시
    /// - PC용 : https://dart.fss.or.kr/dsaf001/main.do?rcpNo=접수번호
    pub rcept_no: String,

    /// ### 공시 제출인명
    pub flr_nm: String,

    /// ### 접수일자
    /// 공시 접수일자(YYYYMMDD)
    pub rcept_dt: String,

    /// ### 비고
    /// 조합된 문자로 각각은 아래와 같은 의미가 있음
    /// - 유 : 본 공시사항은 한국거래소 유가증권시장본부 소관임
    /// - 코 : 본 공시사항은 한국거래소 코스닥시장본부 소관임
    /// - 채 : 본 문서는 한국거래소 채권상장법인 공시사항임
    /// - 넥 : 본 문서는 한국거래소 코넥스시장 소관임
    /// - 공 : 본 공시사항은 공정거래위원회 소관임
    /// - 연 : 본 보고서는 연결부분을 포함한 것임
    /// - 정 : 본 보고서 제출 후 정정신고가 있으니 관련 보고서를 참조하시기 바람
    /// - 철 : 본 보고서는 철회(간주)되었으니 관련 철회신고서(철회간주안내)를 참고하시기 바람
    pub rm: String,
}
