use self::Inner::*;
use crate::assert_impl_all_commons;
use derive_more::{AsRef, Display, FromStr};
use serde::{Deserialize, Serialize};

assert_impl_all_commons!(CorpCls);

/// ### 법인구분
///
/// - Y : 유가
/// - K : 코스닥
/// - N : 코넥스
/// - E : 기타
///
/// ※ 없으면 전체조회, 복수조건 불가
#[derive(Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq, AsRef, Display)]
pub struct CorpCls(Inner);

impl CorpCls {
    pub const Y: Self = Self(Y);
    pub const K: Self = Self(K);
    pub const N: Self = Self(N);
    pub const E: Self = Self(E);
}

impl FromStr for CorpCls {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Y" => Ok(Self::Y),
            "K" => Ok(Self::K),
            "N" => Ok(Self::N),
            "E" => Ok(Self::E),
            _ => Err(format!("Invalid CorpCls: {}", s)),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq, Display, FromStr)]
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
    fn corp_cls_display() {
        assert_eq!(CorpCls::Y.to_string(), "Y");
        assert_eq!(CorpCls::K.to_string(), "K");
        assert_eq!(CorpCls::N.to_string(), "N");
        assert_eq!(CorpCls::E.to_string(), "E");
    }

    #[test]
    fn corp_cls_from_str() {
        assert_eq!("Y".parse(), Ok(CorpCls::Y));
        assert_eq!("K".parse(), Ok(CorpCls::K));
        assert_eq!("N".parse(), Ok(CorpCls::N));
        assert_eq!("E".parse(), Ok(CorpCls::E));
    }
}
