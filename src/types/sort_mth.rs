use crate::assert_impl_commons_without_default;
use derive_more::{AsMut, AsRef, Display, FromStr};
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use test_variants::generate_consts;

assert_impl_commons_without_default!(SortMth);

/// ### 정렬방법
///
/// - asc : 오름차순
/// - desc : 내림차순
///
/// ※ 기본값 : desc
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize, AsRef, AsMut)]
pub struct SortMth(Inner);

impl Display for SortMth {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[cfg(test)]
use crate::test_utils::MockDefault;
#[cfg(test)]
impl MockDefault for SortMth {
    fn mock_default() -> Self {
        Self(Inner::Dsc)
    }
}

#[allow(non_upper_case_globals)]
#[derive(
    Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Display, Serialize, Deserialize, FromStr,
)]
#[serde(rename_all = "lowercase")]
#[display("{_variant}")]
#[generate_consts(SortMth)]
enum Inner {
    Asc,
    Dsc,
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Context;

    #[test]
    fn serialize() -> anyhow::Result<()> {
        let sort = SortMth::ASC;
        let serialized = serde_json::to_string(&sort).context("Failed to serialize")?;
        assert_eq!(serialized, r#""asc""#);
        Ok(())
    }

    #[test]
    fn deserialize() -> anyhow::Result<()> {
        let deserialized: SortMth =
            serde_json::from_str(r#""asc""#).context("Failed to deserialize")?;
        assert_eq!(deserialized, SortMth::ASC);
        Ok(())
    }

    #[test]
    fn display() {
        assert_eq!(SortMth::ASC.to_string(), "Asc");
    }
}
