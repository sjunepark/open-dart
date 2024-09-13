use crate::assert_impl_commons_without_default;
use derive_more::{AsMut, AsRef, Display, FromStr};
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use test_variants::{generate_consts, test_variants};

assert_impl_commons_without_default!(CorpCls);

/// ### 법인구분
///
/// - Y : 유가
/// - K : 코스닥
/// - N : 코넥스
/// - E : 기타
///
/// ※ 없으면 전체조회, 복수조건 불가
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize, AsRef, AsMut)]
pub struct CorpCls(Inner);

impl Display for CorpCls {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(
    Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Display, Serialize, Deserialize, FromStr,
)]
#[display("{_variant}")]
#[test_variants(CorpCls)]
#[generate_consts(CorpCls)]
enum Inner {
    /// 유가
    Y,
    /// 코스닥
    K,
    /// 코넥스
    N,
    /// 기타
    E,
}
