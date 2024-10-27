use crate::assert_impl_commons_without_default;
use crate::error::{OpenDartError, ValidationError};
use std::num::ParseIntError;

use derive_more::{AsRef, Display, From, Into};
use serde::{Deserialize, Serialize};

/// ### 사업연도(4자리)
/// ※ 2015년 이후 부터 정보제공
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
pub struct BsnsYear(String);
assert_impl_commons_without_default!(BsnsYear);

impl BsnsYear {
    pub fn try_new(value: &str) -> Result<Self, OpenDartError> {
        let value: u16 = value.parse().map_err(|e: ParseIntError| ValidationError {
            value: value.to_string(),
            message: e.to_string(),
        })?;

        if 2015 <= value {
            Ok(Self(value.to_string()))
        } else {
            Err(ValidationError {
                value: value.to_string(),
                message: "Year must be greater than or equal to 2015".to_string(),
            })?
        }
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

#[cfg(test)]
impl crate::test_utils::MockDefault for BsnsYear {
    fn mock_default() -> Self {
        let name = "2023".to_string();
        BsnsYear::try_new(&name)
            .unwrap_or_else(|_| panic!("failed to create BsnsYear with: {}", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let bsns_year = BsnsYear::try_new("2023").expect("failed to create bsns_year");
        let serialized = serde_json::to_string(&bsns_year).expect("failed to serialize");
        assert_eq!(serialized, "\"2023\"");
    }

    #[test]
    fn deserialize() {
        let bsns_year =
            serde_json::from_str::<BsnsYear>("\"2023\"").expect("failed to deserialize");
        assert_eq!(bsns_year.into_inner(), "2023");
    }

    #[test]
    fn try_new_with_empty_string_should_fail() {
        let bsns_year = BsnsYear::try_new("");
        assert!(bsns_year.is_err());
    }
}
