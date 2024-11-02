use crate::utils::derive_enum;

derive_enum! {
    /// ## 정렬
    ///
    /// ※ 기본값 : date
    #[serde(rename_all = "lowercase")]
    pub enum Sort {
        /// 접수일자
        Date,
        /// 회사명
        Crp,
        /// 보고서명
        Rpt,
    }
}

#[cfg(test)]
impl crate::test_utils::MockDefault for Sort {
    fn mock_default() -> Self {
        Self::Date
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let sort = Sort::Date;
        let serialized = serde_json::to_string(&sort).expect("Failed to serialize");
        assert_eq!(serialized, r#""date""#);
    }

    #[test]
    fn deserialize() {
        let deserialized: Sort = serde_json::from_str(r#""date""#).expect("Failed to deserialize");
        dbg!(&deserialized);
    }

    #[test]
    fn display() {
        assert_eq!(Sort::Date.to_string(), "Date");
    }
}
