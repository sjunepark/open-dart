use crate::assert_impl_commons_without_default;
use derive_more::{AsMut, AsRef, Display, FromStr};
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use test_variants::{generate_consts, test_variants};

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
        Self(Inner::dsc)
    }
}

#[allow(non_upper_case_globals)]
#[derive(
    Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Display, Serialize, Deserialize, FromStr,
)]
#[display("{_variant}")]
#[test_variants(SortMth)]
#[generate_consts(SortMth)]
enum Inner {
    asc,
    dsc,
}
