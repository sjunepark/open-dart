use nutype::nutype;

/// ### 고유번호
/// 공시대상회사의 고유번호(8자리)
///
/// ※ 개발가이드 > 공시정보 > 고유번호 참고
#[nutype(
    validate(len_char_min = 8, len_char_max = 8, predicate = is_digits),
    derive(Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq)
)]
pub struct CorpCode(String);

fn is_digits(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corp_code_with_valid_length() {
        let corp_code =
            CorpCode::try_new("12345678".to_string()).expect("failed to create corp_code");
        assert_eq!(corp_code.into_inner(), "12345678");
    }

    #[test]
    fn corp_code_with_invalid_length() {
        let corp_code = CorpCode::try_new("1234567".to_string());
        assert!(corp_code.is_err());
    }

    #[test]
    fn corp_code_with_invalid_char() {
        let corp_code = CorpCode::try_new("1234567a".to_string());
        assert!(corp_code.is_err());
    }
}
