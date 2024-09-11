use nutype::nutype;
use std::fmt::Display;

/// ### 고유번호
/// 공시대상회사의 고유번호(8자리)
///     
/// ※ 개발가이드 > 공시정보 > 고유번호 참고
#[nutype(
    validate(len_char_min = 8, len_char_max = 8, predicate = is_digits),
    derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize, AsRef)
)]
pub struct CorpCode(String);

impl Display for CorpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

fn is_digits(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let corp_code =
            CorpCode::try_new("12345678".to_string()).expect("failed to create corp_code");
        let serialized = serde_json::to_string(&corp_code).expect("failed to serialize");
        assert_eq!(serialized, "\"12345678\"");
    }

    #[test]
    fn deserialize() {
        let corp_code =
            serde_json::from_str::<CorpCode>("\"12345678\"").expect("failed to deserialize");
        assert_eq!(corp_code.into_inner(), "12345678");
    }

    #[test]
    fn try_new_with_valid_length_and_digits_should_succeed() {
        let corp_code =
            CorpCode::try_new("12345678".to_string()).expect("failed to create corp_code");
        assert_eq!(corp_code.into_inner(), "12345678");
    }

    #[test]
    fn try_new_with_whitespace_should_fail() {
        let corp_code = CorpCode::try_new("12345678 ".to_string());
        assert!(corp_code.is_err());
    }

    #[test]
    fn try_new_with_invalid_length_should_fail() {
        let corp_code = CorpCode::try_new("1234567".to_string());
        assert!(corp_code.is_err());
    }

    #[test]
    fn try_new_with_invalid_char_should_fail() {
        let corp_code = CorpCode::try_new("1234567a".to_string());
        assert!(corp_code.is_err());
    }
}
