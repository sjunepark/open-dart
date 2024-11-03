use crate::utils::derive_enum;

derive_enum! {
    /// ## 정렬방법
    ///
    /// ※ 기본값 : desc
    #[serde(rename_all = "lowercase")]
    pub enum SortMth {
        /// 오름차순
        Asc,
        /// 내림차순
        Desc,
    }
}

#[cfg(test)]
impl crate::test_utils::MockDefault for SortMth {
    fn mock_default() -> Self {
        Self::Desc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let sort = SortMth::Asc;
        let serialized = serde_json::to_string(&sort).expect("Failed to serialize");
        assert_eq!(serialized, r#""asc""#);
    }

    #[test]
    fn deserialize() {
        let deserialized: SortMth =
            serde_json::from_str(r#""asc""#).expect("Failed to deserialize");
        assert_eq!(deserialized, SortMth::Asc);
    }

    #[test]
    fn display() {
        assert_eq!(SortMth::Asc.to_string(), "Asc");
    }
}
