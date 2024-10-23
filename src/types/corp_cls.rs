use crate::assert_impl_commons_without_default;
use derive_more::{AsMut, AsRef, Display, FromStr};
use generate_consts::generate_consts;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

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

#[cfg(test)]
use crate::test_utils::MockDefault;
#[cfg(test)]
impl MockDefault for CorpCls {
    fn mock_default() -> Self {
        Self(Inner::Y)
    }
}

#[derive(
    Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Display, Serialize, Deserialize, FromStr,
)]
#[display("{_variant}")]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let corp_cls = CorpCls::Y;
        let serialized = serde_json::to_string(&corp_cls).expect("Failed to serialize");
        assert_eq!(serialized, r#""Y""#);
    }

    #[test]
    fn deserialize() {
        let corp_cls = CorpCls::Y;
        let deserialized: CorpCls = serde_json::from_str(r#""Y""#).expect("Failed to deserialize");
        assert_eq!(deserialized, corp_cls);
    }

    #[test]
    fn display() {
        assert_eq!(CorpCls::Y.to_string(), "Y");
    }
}
