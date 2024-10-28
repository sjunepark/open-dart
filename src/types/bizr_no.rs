use crate::statics::assert_impl_commons_without_default;
use derive_more::{AsRef, Display, From, Into};
use serde::{Deserialize, Serialize};

assert_impl_commons_without_default!(BizrNo);
/// ## 사업자등록번호
///
/// 10자리
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
pub struct BizrNo(String);

impl BizrNo {
    pub fn try_new(value: &str) -> Result<Self, crate::error::OpenDartError> {
        if value.len() == 10 && is_digits(value) {
            Ok(Self(value.to_string()))
        } else {
            Err(crate::error::ValidationError {
                value: value.to_string(),
                message: "bizr_no must be 10 digits".to_string(),
            })?
        }
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

#[cfg(test)]
impl crate::test_utils::MockDefault for BizrNo {
    fn mock_default() -> Self {
        let bizr_no = "1234567890".to_string();
        BizrNo::try_new(&bizr_no)
            .unwrap_or_else(|_| panic!("failed to create BizrNo with: {}", bizr_no))
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
        let bizr_no = BizrNo::try_new("1234567890").expect("failed to create bizr_no");
        let serialized = serde_json::to_string(&bizr_no).expect("failed to serialize");
        assert_eq!(serialized, "\"1234567890\"");
    }

    #[test]
    fn deserialize() {
        let bizr_no =
            serde_json::from_str::<BizrNo>("\"1234567890\"").expect("failed to deserialize");
        assert_eq!(bizr_no.into_inner(), "1234567890");
    }

    #[test]
    fn try_new_with_valid_length_and_digits_should_succeed() {
        let bizr_no = BizrNo::try_new("1234567890").expect("failed to create bizr_no");
        assert_eq!(bizr_no.into_inner(), "1234567890");
    }

    #[test]
    fn try_new_with_whitespace_should_fail() {
        let bizr_no = BizrNo::try_new("1234567890 ");
        assert!(bizr_no.is_err());
    }

    #[test]
    fn try_new_with_invalid_length_should_fail() {
        // Invalid length of 7
        let bizr_no = BizrNo::try_new("1234567");
        assert!(bizr_no.is_err());
    }

    #[test]
    fn try_new_with_invalid_char_should_fail() {
        let bizr_no = BizrNo::try_new("00593a");
        assert!(bizr_no.is_err());
    }
}
