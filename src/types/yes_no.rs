use crate::assert_impl_commons_without_default;
use derive_more::{AsMut, AsRef, Display, FromStr};
use generate_consts::generate_consts;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

assert_impl_commons_without_default!(YesNo);

/// ### 최종보고서 검색여부
/// 최종보고서만 검색여부(Y or N)
///
/// - 기본값 : N(정정이 있는 경우 최종정정만 검색)
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize, AsRef, AsMut)]
pub struct YesNo(Inner);

impl Display for YesNo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[cfg(test)]
use crate::test_utils::MockDefault;
#[cfg(test)]
impl MockDefault for YesNo {
    fn mock_default() -> Self {
        Self(Inner::Y)
    }
}

#[derive(
    Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Display, Serialize, Deserialize, FromStr,
)]
#[display("{_variant}")]
#[generate_consts(YesNo)]
enum Inner {
    Y,
    N,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let yes_no = YesNo::Y;
        let serialized = serde_json::to_string(&yes_no).expect("Failed to serialize");
        assert_eq!(serialized, r#""Y""#);
    }

    #[test]
    fn deserialize() {
        let deserialized: YesNo = serde_json::from_str(r#""Y""#).expect("Failed to deserialize");
        assert_eq!(deserialized, YesNo::Y);
    }

    #[test]
    fn display() {
        assert_eq!(YesNo::Y.to_string(), "Y");
    }
}
