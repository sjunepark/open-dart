use self::Inner::*;
use crate::assert_impl_all_commons;
use crate::error::OpenDartError;
use derive_more::{AsMut, AsRef, Display, FromStr};
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

assert_impl_all_commons!(CorpCls);

/// ### 법인구분
///
/// - Y : 유가
/// - K : 코스닥
/// - N : 코넥스
/// - E : 기타
///
/// ※ 없으면 전체조회, 복수조건 불가
#[derive(
    Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Serialize, Deserialize, AsRef, AsMut,
)]
pub struct CorpCls(Option<Inner>);

impl CorpCls {
    pub const Y: Self = Self(Some(Y));
    pub const K: Self = Self(Some(K));
    pub const N: Self = Self(Some(N));
    pub const E: Self = Self(Some(E));
    pub const NONE: Self = Self(None);
}

impl Display for CorpCls {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(inner) => inner.fmt(f),
            None => write!(f, "None"),
        }
    }
}

impl FromStr for CorpCls {
    type Err = OpenDartError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Y" => Ok(Self::Y),
            "K" => Ok(Self::K),
            "N" => Ok(Self::N),
            "E" => Ok(Self::E),
            "" => Ok(Self::NONE),
            _ => Err(OpenDartError::InvalidArgument(s.to_string())),
        }
    }
}

#[derive(
    Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Display, Serialize, Deserialize, FromStr,
)]
#[display("{_variant}")]
enum Inner {
    Y,
    K,
    N,
    E,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        assert_eq!(CorpCls::default(), CorpCls::NONE);
    }

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::to_string(&CorpCls::Y).expect("Failed to serialize"),
            r#""Y""#
        );
        assert_eq!(
            serde_json::to_string(&CorpCls::K).expect("Failed to serialize"),
            r#""K""#
        );
        assert_eq!(
            serde_json::to_string(&CorpCls::N).expect("Failed to serialize"),
            r#""N""#
        );
        assert_eq!(
            serde_json::to_string(&CorpCls::E).expect("Failed to serialize"),
            r#""E""#
        );
        assert_eq!(
            serde_json::to_string(&CorpCls::NONE).expect("Failed to serialize"),
            r#"null"#
        );
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            serde_json::from_str::<CorpCls>(r#""Y""#).expect("Failed to deserialize"),
            CorpCls::Y
        );
        assert_eq!(
            serde_json::from_str::<CorpCls>(r#""K""#).expect("Failed to deserialize"),
            CorpCls::K
        );
        assert_eq!(
            serde_json::from_str::<CorpCls>(r#""N""#).expect("Failed to deserialize"),
            CorpCls::N
        );
        assert_eq!(
            serde_json::from_str::<CorpCls>(r#""E""#).expect("Failed to deserialize"),
            CorpCls::E
        );
        assert_eq!(
            serde_json::from_str::<CorpCls>(r#"null"#).expect("Failed to deserialize"),
            CorpCls::NONE
        );
    }

    #[test]
    fn display() {
        assert_eq!(CorpCls::Y.to_string(), "Y");
        assert_eq!(CorpCls::K.to_string(), "K");
        assert_eq!(CorpCls::N.to_string(), "N");
        assert_eq!(CorpCls::E.to_string(), "E");
        assert_eq!(CorpCls::NONE.to_string(), "None");
    }

    #[test]
    fn from_str() {
        assert!(matches!("Y".parse::<CorpCls>(), Ok(CorpCls::Y)));
        assert!(matches!("K".parse::<CorpCls>(), Ok(CorpCls::K)));
        assert!(matches!("N".parse::<CorpCls>(), Ok(CorpCls::N)));
        assert!(matches!("E".parse::<CorpCls>(), Ok(CorpCls::E)));
        assert!(matches!("".parse::<CorpCls>(), Ok(CorpCls::NONE)));
    }
}
