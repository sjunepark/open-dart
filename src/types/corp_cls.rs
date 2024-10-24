use derive_more::Display;
use serde::{Deserialize, Serialize};

use crate::assert_impl_commons_without_default;

/// ### 법인구분
///
/// - Y : 유가
/// - K : 코스닥
/// - N : 코넥스
/// - E : 기타
///
/// ※ 없으면 전체조회, 복수조건 불가
#[derive(
    Debug,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    // derive_more
    Display,
    // serde
    Serialize,
    Deserialize,
)]
pub enum CorpCls {
    /// 유가
    Y,
    /// 코스닥
    K,
    /// 코넥스
    N,
    /// 기타
    E,
}
assert_impl_commons_without_default!(CorpCls);

#[cfg(test)]
impl crate::test_utils::MockDefault for CorpCls {
    fn mock_default() -> Self {
        Self::Y
    }
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
