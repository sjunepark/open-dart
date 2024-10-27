use derive_more::Display;
use serde::{Deserialize, Serialize};

use crate::statics::assert_impl_commons_without_default;

assert_impl_commons_without_default!(PblntfTy);
/// ### 공시유형
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
pub enum PblntfTy {
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
impl crate::test_utils::MockDefault for PblntfTy {
    fn mock_default() -> Self {
        Self::F
    }
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
