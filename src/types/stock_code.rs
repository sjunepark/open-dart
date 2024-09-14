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
use crate::test_utils::MockDefault;
#[cfg(test)]
impl MockDefault for StockCode {
    fn mock_default() -> Self {
        let stock_code = "005930".to_string();
        StockCode::try_new(&stock_code)
            .unwrap_or_else(|_| panic!("failed to create StockCode with code: {}", stock_code))
    }
}

fn is_digits(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Context;

    #[test]
    fn serialize() -> anyhow::Result<()> {
        let stock_coe =
            StockCode::try_new("005930".to_string()).context("failed to create stock_coe")?;
        let serialized = serde_json::to_string(&stock_coe).context("failed to serialize")?;
        assert_eq!(serialized, "\"005930\"");
        Ok(())
    }

    #[test]
    fn deserialize() -> anyhow::Result<()> {
        let stock_coe =
            serde_json::from_str::<StockCode>("\"005930\"").context("failed to deserialize")?;
        assert_eq!(stock_coe.into_inner(), "005930");
        Ok(())
    }

    #[test]
    fn try_new_with_valid_length_and_digits_should_succeed() -> anyhow::Result<()> {
        let stock_coe =
            StockCode::try_new("005930".to_string()).context("failed to create stock_coe")?;
        assert_eq!(stock_coe.into_inner(), "005930");
        Ok(())
    }

    #[test]
    fn try_new_with_whitespace_should_fail() -> anyhow::Result<()> {
        let stock_coe = StockCode::try_new("005930 ".to_string());
        assert!(stock_coe.is_err());
        Ok(())
    }

    #[test]
    fn try_new_with_invalid_length_should_fail() -> anyhow::Result<()> {
        let stock_coe = StockCode::try_new("0059301".to_string());
        assert!(stock_coe.is_err());
        Ok(())
    }

    #[test]
    fn try_new_with_invalid_char_should_fail() -> anyhow::Result<()> {
        let stock_coe = StockCode::try_new("00593a".to_string());
        assert!(stock_coe.is_err());
        Ok(())
    }
}
