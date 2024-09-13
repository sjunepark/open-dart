use crate::assert_impl_commons_without_default;
use derive_more::{AsMut, AsRef, Display};
use serde::{Deserialize, Serialize};

assert_impl_commons_without_default!(PblntfTy);

/// ### 공시유형
///
/// - A : 정기공시
/// - B : 주요사항보고
/// - C : 발행공시
/// - D : 지분공시
/// - E : 기타공시
/// - F : 외부감사관련
/// - G : 펀드공시
/// - H : 자산유동화
/// - I : 거래소공시
/// - J : 공정위공시
#[derive(
    Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Display, Serialize, Deserialize, AsMut, AsRef,
)]
pub struct PblntfTy(Inner);

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Display, Serialize, Deserialize)]
#[display("{_variant}")]
enum Inner {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
}
