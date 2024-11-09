use crate::error::{MyValidationError, OpenDartError};
use crate::utils::derive_newtype;
use std::num::ParseIntError;

derive_newtype! {
    /// ## 사업연도(4자리)
    ///
    /// ※ 2015년 이후 부터 정보제공
    pub struct BsnsYear(String);
}

impl BsnsYear {
    pub fn try_new(value: &str) -> Result<Self, OpenDartError> {
        let value: u16 = value
            .parse()
            .map_err(|e: ParseIntError| MyValidationError {
                value: value.to_string(),
                message: e.to_string(),
            })?;

        if 2015 <= value {
            Ok(Self(value.to_string()))
        } else {
            Err(MyValidationError {
                value: value.to_string(),
                message: "Year must be greater than or equal to 2015".to_string(),
            })?
        }
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl TryFrom<&str> for BsnsYear {
    type Error = OpenDartError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        BsnsYear::try_new(value)
    }
}

#[cfg(test)]
impl crate::test_utils::MockDefault for BsnsYear {
    fn mock_default() -> Self {
        let name = "2023".to_string();
        BsnsYear::try_new(&name)
            .unwrap_or_else(|_| panic!("failed to create BsnsYear with: {}", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let bsns_year = BsnsYear::try_new("2023").expect("failed to create bsns_year");
        let serialized = serde_json::to_string(&bsns_year).expect("failed to serialize");
        assert_eq!(serialized, "\"2023\"");
    }

    #[test]
    fn deserialize() {
        let bsns_year =
            serde_json::from_str::<BsnsYear>("\"2023\"").expect("failed to deserialize");
        assert_eq!(bsns_year.into_inner(), "2023");
    }

    #[test]
    fn try_new_with_empty_string_should_fail() {
        let bsns_year = BsnsYear::try_new("");
        assert!(bsns_year.is_err());
    }
}
