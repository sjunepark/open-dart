use crate::utils::derive_enum;

derive_enum! {
    pub enum YesNo {
        /// Yes
        Y,
        /// No
        N,
    }
}

#[cfg(test)]
impl crate::test_utils::MockDefault for YesNo {
    fn mock_default() -> Self {
        Self::Y
    }
}

// region: Implementations

/// ## 최종보고서 검색여부
/// 최종보고서만 검색여부(Y or N)
///
/// - 기본값 : N(정정이 있는 경우 최종정정만 검색)
pub type LastReprtAt = YesNo;

// endregion: Implementations

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let yes_no = YesNo::Y;
        let serialized = serde_json::to_string(&yes_no).expect("Failed to serialize");
        assert_eq!(serialized, r#""Y""#);
    }

    #[test]
    fn deserialize() {
        let deserialized: YesNo = serde_json::from_str(r#""Y""#).expect("Failed to deserialize");
        assert_eq!(deserialized, YesNo::Y);
    }

    #[test]
    fn display() {
        assert_eq!(YesNo::Y.to_string(), "Y");
    }
}
