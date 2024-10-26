use derive_more::Display;
use serde::{Deserialize, Serialize};

use crate::assert_impl_commons_without_default;

assert_impl_commons_without_default!(ReprtCode);

/// ### 보고서 코드
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
pub enum ReprtCode {
    /// 1분기보고서
    Q1,
    /// 반기보고서
    Q2,
    /// 3분기보고서
    Q3,
    /// 사업보고서
    YE,
}

#[cfg(test)]
impl crate::test_utils::MockDefault for ReprtCode {
    fn mock_default() -> Self {
        Self::YE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let reprt_code = ReprtCode::YE;
        let serialized = serde_json::to_string(&reprt_code).expect("Failed to serialize");
        assert_eq!(serialized, r#""YE""#);
    }

    #[test]
    fn deserialize() {
        let reprt_code = ReprtCode::YE;
        let deserialized: ReprtCode =
            serde_json::from_str(r#""YE""#).expect("Failed to deserialize");
        assert_eq!(deserialized, reprt_code);
    }

    #[test]
    fn display() {
        assert_eq!(ReprtCode::YE.to_string(), "YE");
    }
}
