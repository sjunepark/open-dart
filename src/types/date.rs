use crate::error::OpenDartError;
use crate::utils::derive_newtype;
use chrono::NaiveDate;
use std::str::FromStr;

derive_newtype! {
    /// ## YYYYMMDD
    pub struct Date(#[serde(with = "dart_date_format")] NaiveDate);
}

impl Date {
    pub fn new(date: NaiveDate) -> Self {
        Self(date)
    }
}

impl FromStr for Date {
    type Err = OpenDartError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let date = NaiveDate::parse_from_str(s, "%Y%m%d")?;
        Ok(Date::new(date))
    }
}

impl TryFrom<&str> for Date {
    type Error = OpenDartError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Date::from_str(value)
    }
}

mod dart_date_format {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y%m%d";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(dt)
    }
}

#[cfg(test)]
impl crate::test_utils::MockDefault for Date {
    fn mock_default() -> Self {
        let today = chrono::Local::now().naive_local().date();
        let year_before = today - chrono::Duration::days(365);
        Date::new(year_before)
    }
}

// region: Implementations

/// ## 시작일
/// 검색시작 접수일자(YYYYMMDD)
///
/// - 기본값 : 종료일(end_de)
/// - 고유번호(corp_code)가 없는 경우 검색기간은 3개월로 제한
pub type BgnDe = Date;

/// ## 종료일
/// 검색종료 접수일자(YYYYMMDD)
///
/// - 기본값 : 당일
/// - 고유번호(corp_code)가 없는 경우 검색기간은 3개월로 제한
pub type EndDe = Date;

/// ## 설립일
///
/// YYYYMMDD
pub type EstDt = Date;

/// ### 접수일자
/// 공시 접수일자(YYYYMMDD)
pub type RceptDt = Date;

// endregion: Implementations

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let date = NaiveDate::from_ymd_opt(2021, 1, 1).expect("failed to create NaiveDate");
        let date = Date::new(date);
        let serialized = serde_json::to_string(&date).expect("failed to serialize");
        assert_eq!(serialized, "\"20210101\"");
    }

    #[test]
    fn deserialize() {
        let date = serde_json::from_str::<Date>("\"20210101\"").expect("failed to deserialize");
        let expected_date =
            NaiveDate::from_ymd_opt(2021, 1, 1).expect("failed to create NaiveDate");
        assert_eq!(date, expected_date.into());
    }

    #[test]
    fn try_new_with_valid_date_should_succeed() {
        let expected_date =
            NaiveDate::from_ymd_opt(2021, 1, 1).expect("failed to create NaiveDate");
        let date = Date::new(expected_date);
        assert_eq!(date, expected_date.into());
    }

    #[test]
    fn from_str_with_valid_date_should_succeed() {
        let date = Date::from_str("20210101").expect("failed to create Date");
        let expected_date =
            NaiveDate::from_ymd_opt(2021, 1, 1).expect("failed to create NaiveDate");
        assert_eq!(date, expected_date.into());
    }

    #[test]
    fn from_str_with_invalid_date_should_fail() {
        let date = Date::from_str("20231232");
        assert!(date.is_err());
    }

    #[test]
    fn deserialize_invalid_date_should_fail() {
        let result = serde_json::from_str::<Date>("\"18991232\"");
        assert!(result.is_err());
    }
}
