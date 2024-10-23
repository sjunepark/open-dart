use crate::assert_impl_commons_without_default;
use derive_more::{AsMut, AsRef, Display, FromStr};
use generate_consts::generate_consts;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

assert_impl_commons_without_default!(Sort);

/// ### 정렬
///
/// - date : 접수일자
/// - crp : 회사명
/// - rpt : 보고서명
///
/// ※ 기본값 : date
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize, AsRef, AsMut)]
pub struct Sort(Inner);

impl Display for Sort {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[cfg(test)]
use crate::test_utils::MockDefault;

#[cfg(test)]
impl MockDefault for Sort {
    fn mock_default() -> Self {
        Self(Inner::Date)
    }
}

#[derive(
    Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Display, Serialize, Deserialize, FromStr,
)]
#[serde(rename_all = "lowercase")]
#[display("{_variant}")]
#[generate_consts(Sort)]
enum Inner {
    Date,
    Crp,
    Rpt,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let sort = Sort::DATE;
        let serialized = serde_json::to_string(&sort).expect("Failed to serialize");
        assert_eq!(serialized, r#""date""#);
    }

    #[test]
    fn deserialize() {
        let deserialized: Sort = serde_json::from_str(r#""date""#).expect("Failed to deserialize");
        dbg!(&deserialized);
    }

    #[test]
    fn display() {
        assert_eq!(Sort::DATE.to_string(), "Date");
    }
}
