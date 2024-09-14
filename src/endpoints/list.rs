//! ## 공시검색
//! [link](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS001&apiId=2019001)
//! 공시 유형별, 회사별, 날짜별 등 여러가지 조건으로 공시보고서 검색기능을 제공합니다.
use crate::assert_impl_commons;
use crate::error::OpenDartError;
use crate::types::CorpCls;
use crate::types::CorpCode;
use crate::types::CrtfcKey;
use crate::types::PblntfDetailTy;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

assert_impl_commons!(ListRequestParams);

// region: Request Params
/// Documentation exists in each field's types
#[derive(
    Builder, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize, Default,
)]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenDartError"))]
pub struct ListRequestParams {
    #[builder(setter(skip))]
    crtfc_key: CrtfcKey,

    pub corp_code: Option<CorpCode>,

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
    pub last_reprt_at: Option<char>,

    pub pblntf_ty: Option<char>,

    pub pblntf_detail_ty: Option<PblntfDetailTy>,

    pub corp_cls: Option<CorpCls>,

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

impl std::fmt::Display for ListRequestParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

// endregion

// region: Response
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
pub struct List {
    /// ### 페이지 번호
    page_no: i32,

    /// ### 페이지 별 건수
    page_count: i32,

    /// ### 총 건수
    /// 총 페이지 수
    total_count: i32,

    /// ### 총 페이지 수
    total_page: i32,

    list: Vec<ListCorp>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
struct ListCorp {
    corp_cls: CorpCls,

    /// ### 종목명(법인명)
    /// 공시대상회사의 종목명(상장사) 또는 법인명(기타법인)
    corp_name: String,

    /// ### 고유번호
    /// 공시대상회사의 고유번호(8자리)
    corp_code: CorpCode,

    /// ### 종목코드
    /// 상장회사의 종목코드(6자리)
    stock_code: String,

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
    report_nm: String,

    /// ### 접수번호
    /// 접수번호(14자리)
    ///
    /// ※ 공시뷰어 연결에 이용예시
    /// - PC용 : https://dart.fss.or.kr/dsaf001/main.do?rcpNo=접수번호
    rcept_no: String,

    /// ### 공시 제출인명
    flr_nm: String,

    /// ### 접수일자
    /// 공시 접수일자(YYYYMMDD)
    rcept_dt: String,

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
    rm: String,
}

// endregion

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_request_params_builder_works_with_all_fields_specified() {
        let corp_code = CorpCode::try_new("00120182".to_string()).unwrap();
        let bgn_de = "20220101".to_string();
        let end_de = "20240630".to_string();
        let last_reprt_at = 'Y';
        let pblntf_ty = 'p';
        let pblntf_detail_ty = PblntfDetailTy::A001;
        let corp_cls = CorpCls::Y;
        let sort = "sort".to_string();
        let sort_mth = "sort_mth".to_string();
        let page_no = "page_no".to_string();
        let page_count = "page_count".to_string();

        let params = ListRequestParamsBuilder::default()
            .corp_code(corp_code.clone())
            .bgn_de(&bgn_de)
            .end_de(&end_de)
            .last_reprt_at(last_reprt_at)
            .pblntf_ty(pblntf_ty)
            .pblntf_detail_ty(pblntf_detail_ty.clone())
            .corp_cls(corp_cls.clone())
            .sort(&sort)
            .sort_mth(&sort_mth)
            .page_no(&page_no)
            .page_count(&page_count)
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
}
