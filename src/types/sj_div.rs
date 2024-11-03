use crate::utils::derive_enum;

derive_enum!(
    /// 재무제표구분
    pub enum SjDiv {
        /// 재무상태표
        BS,
        /// 손익계산서
        IS,
        /// 포괄손익계산서
        CIS,
        /// 현금흐름표
        CF,
        /// 자본변동표
        SCE,
    }
);

#[cfg(test)]
impl crate::test_utils::MockDefault for SjDiv {
    fn mock_default() -> Self {
        Self::IS
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let sj_div = SjDiv::IS;
        let serialized = serde_json::to_string(&sj_div).expect("Failed to serialize");
        assert_eq!(serialized, r#""IS""#);
    }

    #[test]
    fn deserialize() {
        let sj_div = SjDiv::IS;
        let deserialized: SjDiv = serde_json::from_str(r#""IS""#).expect("Failed to deserialize");
        assert_eq!(deserialized, sj_div);
    }

    #[test]
    fn display() {
        assert_eq!(SjDiv::IS.to_string(), "IS");
    }
}
