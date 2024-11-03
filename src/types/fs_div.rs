use crate::utils::derive_enum;

derive_enum!(
    /// ## 개별/연결 구분
    pub enum FsDiv {
        /// 재무제표
        OFS,
        /// 연결재무제표
        CFS,
    }
);

#[cfg(test)]
impl crate::test_utils::MockDefault for FsDiv {
    fn mock_default() -> Self {
        Self::OFS
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let fs_div = FsDiv::OFS;
        let serialized = serde_json::to_string(&fs_div).expect("Failed to serialize");
        assert_eq!(serialized, r#""OFS""#);
    }

    #[test]
    fn deserialize() {
        let fs_div = FsDiv::OFS;
        let deserialized: FsDiv = serde_json::from_str(r#""OFS""#).expect("Failed to deserialize");
        assert_eq!(deserialized, fs_div);
    }

    #[test]
    fn display() {
        assert_eq!(FsDiv::OFS.to_string(), "OFS");
    }
}
