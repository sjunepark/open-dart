use crate::assert_impl_commons_without_default;
use nutype::nutype;
use std::fmt::Display;

assert_impl_commons_without_default!(StockCode);

/// ### 종목코드
/// 상장회사의 종목코드(6자리)
#[nutype(
    validate(len_char_min = 6, len_char_max = 6, predicate = is_digits),
    derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize, AsRef)
)]
pub struct StockCode(String);

impl Display for StockCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
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
        let stock_code =
            StockCode::try_new("005930".to_string()).expect("failed to create stock_code");
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
        let stock_code =
            StockCode::try_new("005930".to_string()).expect("failed to create stock_code");
        assert_eq!(stock_code.into_inner(), "005930");
    }

    #[test]
    fn try_new_with_whitespace_should_fail() {
        let stock_code = StockCode::try_new("005930 ".to_string());
        assert!(stock_code.is_err());
    }

    #[test]
    fn try_new_with_invalid_length_should_fail() {
        // Invalid length of 7
        let stock_code = StockCode::try_new("0059301".to_string());
        assert!(stock_code.is_err());
    }

    #[test]
    fn try_new_with_invalid_char_should_fail() {
        let stock_code = StockCode::try_new("00593a".to_string());
        assert!(stock_code.is_err());
    }
}
