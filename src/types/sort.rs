use crate::assert_impl_commons_without_default;
use derive_more::{AsMut, AsRef, Display, FromStr};
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use test_variants::{generate_consts, test_variants};

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
        Self(Inner::date)
    }
}

#[derive(
    Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Display, Serialize, Deserialize, FromStr,
)]
#[display("{_variant}")]
#[test_variants(Sort)]
#[generate_consts(Sort)]
enum Inner {
    date,
    crp,
    rpt,
}
