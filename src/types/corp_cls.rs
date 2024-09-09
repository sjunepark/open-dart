use self::Inner::*;
use derive_more::{AsRef, From};
use serde::{Deserialize, Serialize};

/// ### 법인구분
///
/// - Y : 유가
/// - K : 코스닥
/// - N : 코넥스
/// - E : 기타
///
/// ※ 없으면 전체조회, 복수조건 불가
#[derive(Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq, From, AsRef)]
pub struct CorpCls(Inner);

#[derive(Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq, From)]
enum Inner {
    Y,
    K,
    N,
    E,
}

impl CorpCls {
    pub const Y: Self = Self(Y);
    pub const K: Self = Self(K);
    pub const N: Self = Self(N);
    pub const E: Self = Self(E);
}

// impl FromStr for CorpCls {
//     type Err = OpenDartError;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s {
//             "Y" => Ok(CorpCls::Y),
//             "K" => Ok(CorpCls::K),
//             "N" => Ok(CorpCls::N),
//             "E" => Ok(CorpCls::E),
//             _ => Err(OpenDartError::InvalidArgument(s.to_string())),
//         }
//     }
// }
