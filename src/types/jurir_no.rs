use crate::statics::assert_impl_commons_without_default;
use derive_more::{AsRef, Display, From, Into};
use serde::{Deserialize, Serialize};

assert_impl_commons_without_default!(JurirNo);
/// ## 법인등록번호
///
/// 13자리
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
pub struct JurirNo(String);

impl JurirNo {
    pub fn try_new(value: &str) -> Result<Self, crate::error::OpenDartError> {
        if value.len() == 13 && is_digits(value) {
            Ok(Self(value.to_string()))
        } else {
            Err(crate::error::ValidationError {
                value: value.to_string(),
                message: "jurir_no must be 13 digits".to_string(),
            })?
        }
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

#[cfg(test)]
impl crate::test_utils::MockDefault for JurirNo {
    fn mock_default() -> Self {
        let jurir_no = "1234567890123".to_string();
        JurirNo::try_new(&jurir_no)
            .unwrap_or_else(|_| panic!("failed to create JurirNo with: {}", jurir_no))
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
        let jurir_no = JurirNo::try_new("1234567890123").expect("failed to create jurir_no");
        let serialized = serde_json::to_string(&jurir_no).expect("failed to serialize");
        assert_eq!(serialized, "\"1234567890123\"");
    }

    #[test]
    fn deserialize() {
        let jurir_no =
            serde_json::from_str::<JurirNo>("\"1234567890123\"").expect("failed to deserialize");
        assert_eq!(jurir_no.into_inner(), "1234567890123");
    }

    #[test]
    fn try_new_with_valid_length_and_digits_should_succeed() {
        let jurir_no = JurirNo::try_new("1234567890123").expect("failed to create jurir_no");
        assert_eq!(jurir_no.into_inner(), "1234567890123");
    }

    #[test]
    fn try_new_with_whitespace_should_fail() {
        let jurir_no = JurirNo::try_new("1234567890123 ");
        assert!(jurir_no.is_err());
    }

    #[test]
    fn try_new_with_invalid_length_should_fail() {
        // Invalid length of 7
        let jurir_no = JurirNo::try_new("1234567");
        assert!(jurir_no.is_err());
    }

    #[test]
    fn try_new_with_invalid_char_should_fail() {
        let jurir_no = JurirNo::try_new("00593a");
        assert!(jurir_no.is_err());
    }
}
