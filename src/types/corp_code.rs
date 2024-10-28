use crate::statics::assert_impl_commons_without_default;
use derive_more::{AsRef, Display, From, Into};
use serde::{Deserialize, Serialize};

assert_impl_commons_without_default!(CorpCode);
/// ## 고유번호
/// 공시대상회사의 고유번호(8자리)
///     
/// ※ 개발가이드 > 공시정보 > 고유번호 참고
#[derive(
    Debug,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    // derive_more
    AsRef,
    Display,
    From,
    Into,
    // serde
    Serialize,
    Deserialize,
)]
#[cfg_attr(
    feature = "diesel_newtype",
    derive(diesel_derive_newtype::DieselNewType)
)]
pub struct CorpCode(String);

impl CorpCode {
    pub fn try_new(value: &str) -> Result<Self, String> {
        if value.is_empty() {
            return Err("empty string is not allowed".to_string());
        };
        if value.len() != 8 {
            return Err("length must be 8".to_string());
        };
        if !value.chars().all(|c| c.is_ascii_digit()) {
            return Err("only digits are allowed".to_string());
        };
        Ok(Self(value.to_string()))
    }
}

#[cfg(test)]
impl crate::test_utils::MockDefault for CorpCode {
    fn mock_default() -> Self {
        let corp_code: String = String::from("00120182");
        CorpCode::try_new(&corp_code)
            .unwrap_or_else(|_| panic!("failed to create CorpCode with: {}", corp_code))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let corp_code = CorpCode::try_new("00120182").expect("failed to create corp_code");
        let serialized = serde_json::to_string(&corp_code).expect("failed to serialize");
        assert_eq!(serialized, "\"00120182\"");
    }

    #[test]
    fn deserialize() {
        let corp_code =
            serde_json::from_str::<CorpCode>("\"00120182\"").expect("failed to deserialize");
        assert_eq!(corp_code.as_ref(), "00120182");
    }

    #[test]
    fn try_new_with_valid_length_and_digits_should_succeed() {
        let corp_code = CorpCode::try_new("00120182").expect("failed to create corp_code");
        assert_eq!(corp_code.as_ref(), "00120182");
    }

    #[test]
    fn try_new_with_whitespace_should_fail() {
        let corp_code = CorpCode::try_new("00120182 ");
        assert!(corp_code.is_err());
    }

    #[test]
    fn try_new_with_invalid_length_should_fail() {
        // Invalid length of 7
        let corp_code = CorpCode::try_new("0012018");
        assert!(corp_code.is_err());
    }

    #[test]
    fn try_new_with_invalid_char_should_fail() {
        let corp_code = CorpCode::try_new("0012018a");
        assert!(corp_code.is_err());
    }
}
