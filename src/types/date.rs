use crate::assert_impl_commons_without_default;
use crate::error::OpenDartError;
use chrono::NaiveDate;
use nutype::nutype;
use std::fmt::Display;
use std::str::FromStr;

assert_impl_commons_without_default!(Date);

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
pub struct Date(NaiveDate);

/// Validate the date
/// Accepts only dates up to today
fn validate(date: &NaiveDate) -> bool {
    date <= &chrono::Local::now().naive_local().into()
}

impl FromStr for Date {
    type Err = OpenDartError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let date = NaiveDate::parse_from_str(s, "%Y-%m-%d")
            .map_err(|e| OpenDartError::InvalidArgument(e.to_string()))?;
        Date::try_new(date).map_err(|e| OpenDartError::InvalidArgument(e.to_string()))
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[cfg(test)]
use crate::test_utils::MockDefault;
#[cfg(test)]
impl MockDefault for Date {
    fn mock_default() -> Self {
        let today = chrono::Local::now().naive_local().date();
        let week_before = today - chrono::Duration::days(7);
        Date::try_new(week_before)
            .unwrap_or_else(|_| panic!("failed to create Date with date: {}", week_before))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Context;

    #[test]
    fn serialize() -> anyhow::Result<()> {
        let date = NaiveDate::from_ymd_opt(2021, 1, 1).context("failed to create NaiveDate")?;
        let date = Date::try_new(date).context("failed to create date")?;
        let serialized = serde_json::to_string(&date).context("failed to serialize")?;
        assert_eq!(serialized, "\"2021-01-01\"");
        Ok(())
    }

    #[test]
    fn deserialize() -> anyhow::Result<()> {
        let date =
            serde_json::from_str::<Date>("\"2021-01-01\"").context("failed to deserialize")?;
        let expected_date =
            NaiveDate::from_ymd_opt(2021, 1, 1).context("failed to create NaiveDate")?;
        assert_eq!(date.into_inner(), expected_date);
        Ok(())
    }

    #[test]
    fn try_new_with_valid_date_should_succeed() -> anyhow::Result<()> {
        let expected_date =
            NaiveDate::from_ymd_opt(2021, 1, 1).context("failed to create NaiveDate")?;
        let date = Date::try_new(expected_date).context("failed to create Date")?;
        assert_eq!(date.into_inner(), expected_date);
        Ok(())
    }

    #[test]
    fn try_new_with_future_date_should_fail() -> anyhow::Result<()> {
        let date = chrono::Local::now().naive_local() + chrono::Duration::days(1);
        let date = Date::try_new(date.into());
        assert!(date.is_err());
        Ok(())
    }

    #[test]
    fn from_str_with_valid_date_should_succeed() -> anyhow::Result<()> {
        let date = Date::from_str("2021-01-01")?;
        let expected_date =
            NaiveDate::from_ymd_opt(2021, 1, 1).context("failed to create NaiveDate")?;
        assert_eq!(date.into_inner(), expected_date);
        Ok(())
    }

    #[test]
    fn from_str_with_future_date_should_fail() -> anyhow::Result<()> {
        let date = Date::from_str("9999-12-31");
        assert!(date.is_err());
        Ok(())
    }

    #[test]
    fn from_str_with_invalid_date_should_fail() -> anyhow::Result<()> {
        let date = Date::from_str("2023-12-32");
        assert!(date.is_err());
        Ok(())
    }

    #[test]
    fn deserialize_future_date_should_fail() -> anyhow::Result<()> {
        let result = serde_json::from_str::<Date>("\"9999-12-31\"");
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn deserialize_invalid_date_should_fail() -> anyhow::Result<()> {
        let result = serde_json::from_str::<Date>("\"1899-12-32\"");
        assert!(result.is_err());
        Ok(())
    }
}