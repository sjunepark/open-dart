use crate::assert_impl_commons_without_default;
use crate::error::OpenDartError;
use chrono::NaiveDate;
use nutype::nutype;
use std::fmt::Display;
use std::str::FromStr;

assert_impl_commons_without_default!(BgnDe);

/// ### 시작일
/// 검색시작 접수일자(YYYYMMDD)
///
/// - 기본값 : 종료일(end_de)
/// - 고유번호(corp_code)가 없는 경우 검색기간은 3개월로 제한
#[nutype(
validate(predicate = validate),
derive(
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Debug,
    Serialize,
    Deserialize,
    AsRef
))]
pub struct BgnDe(NaiveDate);

/// Validate the date
/// Accepts only dates up to today
fn validate(date: &NaiveDate) -> bool {
    date <= &chrono::Local::now().naive_local().into()
}

impl FromStr for BgnDe {
    type Err = OpenDartError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let date = NaiveDate::parse_from_str(s, "%Y-%m-%d")
            .map_err(|e| OpenDartError::InvalidArgument(e.to_string()))?;
        BgnDe::try_new(date).map_err(|e| OpenDartError::InvalidArgument(e.to_string()))
    }
}

impl Display for BgnDe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Context;

    #[test]
    fn serialize() -> anyhow::Result<()> {
        let date = NaiveDate::from_ymd_opt(2021, 1, 1).context("failed to create NaiveDate")?;
        let bgn_de = BgnDe::try_new(date).context("failed to create bgn_de")?;
        let serialized = serde_json::to_string(&bgn_de).context("failed to serialize")?;
        assert_eq!(serialized, "\"2021-01-01\"");
        Ok(())
    }

    #[test]
    fn deserialize() -> anyhow::Result<()> {
        let bgn_de =
            serde_json::from_str::<BgnDe>("\"2021-01-01\"").context("failed to deserialize")?;
        let expected_date =
            NaiveDate::from_ymd_opt(2021, 1, 1).context("failed to create NaiveDate")?;
        assert_eq!(bgn_de.into_inner(), expected_date);
        Ok(())
    }

    #[test]
    fn try_new_with_valid_date_should_succeed() -> anyhow::Result<()> {
        let date = NaiveDate::from_ymd_opt(2021, 1, 1).context("failed to create NaiveDate")?;
        let bgn_de = BgnDe::try_new(date).context("failed to create BgnDe")?;
        assert_eq!(bgn_de.into_inner(), date);
        Ok(())
    }

    #[test]
    fn try_new_with_future_date_should_fail() -> anyhow::Result<()> {
        let date = chrono::Local::now().naive_local() + chrono::Duration::days(1);
        let bgn_de = BgnDe::try_new(date.into());
        assert!(bgn_de.is_err());
        Ok(())
    }

    #[test]
    fn try_new_with_string_should_succeed() -> anyhow::Result<()> {
        let bgn_de = BgnDe::from_str("2021-01-01")?;
        let expected_date =
            NaiveDate::from_ymd_opt(2021, 1, 1).context("failed to create NaiveDate")?;
        assert_eq!(bgn_de.into_inner(), expected_date);
        Ok(())
    }

    #[test]
    fn deserialize_invalid_date_should_fail() -> anyhow::Result<()> {
        let result = serde_json::from_str::<BgnDe>("\"1899-12-32\"");
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn deserialize_invalid_format_should_fail() -> anyhow::Result<()> {
        let result = serde_json::from_str::<BgnDe>("\"2021-13-01\""); // Invalid month
        assert!(result.is_err());
        Ok(())
    }
}
