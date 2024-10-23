use crate::assert_impl_commons_without_default;
use derive_more::{AsMut, AsRef, Display};
use generate_consts::generate_consts;
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

#[cfg(test)]
use crate::test_utils::MockDefault;

#[cfg(test)]
impl MockDefault for PblntfTy {
    fn mock_default() -> Self {
        Self(Inner::F)
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Display, Serialize, Deserialize)]
#[display("{_variant}")]
#[generate_consts(PblntfTy)]
enum Inner {
    /// 정기공시
    A,
    /// 주요사항보고
    B,
    /// 발행공시
    C,
    /// 지분공시
    D,
    /// 기타공시
    E,
    /// 외부감사관련
    F,
    /// 펀드공시
    G,
    /// 자산유동화
    H,
    /// 거래소공시
    I,
    /// 공정위공시
    J,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let pblntf_ty = PblntfTy::F;
        let serialized = serde_json::to_string(&pblntf_ty).expect("Failed to serialize");
        assert_eq!(serialized, r#""F""#);
    }

    #[test]
    fn deserialize() {
        let pblntf_ty = PblntfTy::F;
        let deserialized: PblntfTy = serde_json::from_str(r#""F""#).expect("Failed to deserialize");
        assert_eq!(deserialized, pblntf_ty);
    }

    #[test]
    fn display() {
        assert_eq!(PblntfTy::F.to_string(), "F");
    }
}
