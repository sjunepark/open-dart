use nutype::nutype;

// region: CorpCode
/// ### 고유번호
/// 공시대상회사의 고유번호(8자리)
///
/// ※ 개발가이드 > 공시정보 > 고유번호 참고
#[nutype(
    validate(len_char_min = 8, len_char_max = 8),
    derive(Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq)
)]
pub struct CorpCode(String);
