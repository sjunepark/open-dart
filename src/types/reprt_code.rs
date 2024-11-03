use crate::utils::derive_enum;

derive_enum! {
    /// ## 보고서 코드
    pub enum ReprtCode {
        /// 1분기보고서
        #[serde(rename = "11013")]
        Q1,
        /// 반기보고서
        #[serde(rename = "11012")]
        Q2,
        /// 3분기보고서
        #[serde(rename = "11014")]
        Q3,
        /// 사업보고서
        #[serde(rename = "11011")]
        YE,
    }
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
        let q1 = ReprtCode::Q1;
        let serialized = serde_json::to_string(&q1).expect("Failed to serialize");
        assert_eq!(serialized, r#""11013""#);

        let q2 = ReprtCode::Q2;
        let serialized = serde_json::to_string(&q2).expect("Failed to serialize");
        assert_eq!(serialized, r#""11012""#);

        let q3 = ReprtCode::Q3;
        let serialized = serde_json::to_string(&q3).expect("Failed to serialize");
        assert_eq!(serialized, r#""11014""#);

        let ye = ReprtCode::YE;
        let serialized = serde_json::to_string(&ye).expect("Failed to serialize");
        assert_eq!(serialized, r#""11011""#);
    }

    #[test]
    fn deserialize() {
        let q1 = ReprtCode::Q1;
        let deserialized =
            serde_json::from_str::<ReprtCode>(r#""11013""#).expect("Failed to deserialize");
        assert_eq!(deserialized, q1);

        let q2 = ReprtCode::Q2;
        let deserialized =
            serde_json::from_str::<ReprtCode>(r#""11012""#).expect("Failed to deserialize");
        assert_eq!(deserialized, q2);

        let q3 = ReprtCode::Q3;
        let deserialized =
            serde_json::from_str::<ReprtCode>(r#""11014""#).expect("Failed to deserialize");
        assert_eq!(deserialized, q3);

        let ye = ReprtCode::YE;
        let deserialized =
            serde_json::from_str::<ReprtCode>(r#""11011""#).expect("Failed to deserialize");
        assert_eq!(deserialized, ye);
    }

    #[test]
    fn display() {
        assert_eq!(ReprtCode::YE.to_string(), "YE");
    }
}
