use crate::statics::assert_impl_commons_without_default;
use derive_more::{AsRef, Display, From, Into};
use serde::{Deserialize, Serialize};

assert_impl_commons_without_default!(StockCode);
/// ### 종목코드
/// 상장회사의 종목코드(6자리)
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
pub struct StockCode(String);

impl StockCode {
    pub fn try_new(value: &str) -> Result<Self, crate::error::OpenDartError> {
        if value.len() == 6 && is_digits(value) {
            Ok(Self(value.to_string()))
        } else {
            Err(crate::error::ValidationError {
                value: value.to_string(),
                message: "stock_code must be 6 digits".to_string(),
            })?
        }
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

#[cfg(test)]
impl crate::test_utils::MockDefault for StockCode {
    fn mock_default() -> Self {
        let stock_code = "005930".to_string();
        StockCode::try_new(&stock_code)
            .unwrap_or_else(|_| panic!("failed to create StockCode with: {}", stock_code))
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
        let stock_code = StockCode::try_new("005930").expect("failed to create stock_code");
        let serialized = serde_json::to_string(&stock_code).expect("failed to serialize");
        assert_eq!(serialized, "\"005930\"");
    }

    #[test]
    fn deserialize() {
        let stock_code =
            serde_json::from_str::<StockCode>("\"005930\"").expect("failed to deserialize");
        assert_eq!(stock_code.into_inner(), "005930");
    }

    #[test]
    fn try_new_with_valid_length_and_digits_should_succeed() {
        let stock_code = StockCode::try_new("005930").expect("failed to create stock_code");
        assert_eq!(stock_code.into_inner(), "005930");
    }

    #[test]
    fn try_new_with_whitespace_should_fail() {
        let stock_code = StockCode::try_new("005930 ");
        assert!(stock_code.is_err());
    }

    #[test]
    fn try_new_with_invalid_length_should_fail() {
        // Invalid length of 7
        let stock_code = StockCode::try_new("0059301");
        assert!(stock_code.is_err());
    }

    #[test]
    fn try_new_with_invalid_char_should_fail() {
        let stock_code = StockCode::try_new("00593a");
        assert!(stock_code.is_err());
    }
}
