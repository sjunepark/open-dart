use crate::statics::assert_impl_commons_without_default;
use derive_more::{AsRef, Display, From, Into};
use serde::{Deserialize, Serialize};

assert_impl_commons_without_default!(IndustryCode);
/// ## 업종코드
///
/// 3자리
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
pub struct IndustryCode(String);

impl IndustryCode {
    pub fn try_new(value: &str) -> Result<Self, crate::error::OpenDartError> {
        if value.len() == 3 && is_digits(value) {
            Ok(Self(value.to_string()))
        } else {
            Err(crate::error::ValidationError {
                value: value.to_string(),
                message: "industry_code must be 3 digits".to_string(),
            })?
        }
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

#[cfg(test)]
impl crate::test_utils::MockDefault for IndustryCode {
    fn mock_default() -> Self {
        let industry_code = "005930".to_string();
        IndustryCode::try_new(&industry_code)
            .unwrap_or_else(|_| panic!("failed to create IndustryCode with: {}", industry_code))
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
        let industry_code = IndustryCode::try_new("123").expect("failed to create industry_code");
        let serialized = serde_json::to_string(&industry_code).expect("failed to serialize");
        assert_eq!(serialized, "\"123\"");
    }

    #[test]
    fn deserialize() {
        let industry_code =
            serde_json::from_str::<IndustryCode>("\"123\"").expect("failed to deserialize");
        assert_eq!(industry_code.into_inner(), "123");
    }

    #[test]
    fn try_new_with_valid_length_and_digits_should_succeed() {
        let industry_code = IndustryCode::try_new("123").expect("failed to create industry_code");
        assert_eq!(industry_code.into_inner(), "123");
    }

    #[test]
    fn try_new_with_whitespace_should_fail() {
        let industry_code = IndustryCode::try_new("123 ");
        assert!(industry_code.is_err());
    }

    #[test]
    fn try_new_with_invalid_length_should_fail() {
        // Invalid length of 7
        let industry_code = IndustryCode::try_new("1234567");
        assert!(industry_code.is_err());
    }

    #[test]
    fn try_new_with_invalid_char_should_fail() {
        let industry_code = IndustryCode::try_new("00593a");
        assert!(industry_code.is_err());
    }
}
