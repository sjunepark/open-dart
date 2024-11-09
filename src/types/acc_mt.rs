use crate::error::MyValidationError;
use crate::utils::derive_newtype;

derive_newtype! {
    /// ## 결산월
    ///
    /// MM
    pub struct AccMt(String);
}

impl AccMt {
    pub fn try_new(value: &str) -> Result<Self, crate::error::OpenDartError> {
        let error = MyValidationError {
            value: value.to_string(),
            message: "acc_mt must be of format MM(00~12)".to_string(),
        };

        let month = value.parse::<u8>().map_err(|_| error.clone())?;
        if (1..=12).contains(&month) && value.len() == 2 {
            Ok(Self(value.to_string()))
        } else {
            Err(error)?
        }
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

#[cfg(test)]
impl crate::test_utils::MockDefault for AccMt {
    fn mock_default() -> Self {
        let acc_mt = "01".to_string();
        AccMt::try_new(&acc_mt)
            .unwrap_or_else(|_| panic!("failed to create AccMt with: {}", acc_mt))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let acc_mt = AccMt::try_new("01").expect("failed to create acc_mt");
        let serialized = serde_json::to_string(&acc_mt).expect("failed to serialize");
        assert_eq!(serialized, "\"01\"");
    }

    #[test]
    fn deserialize() {
        let acc_mt = serde_json::from_str::<AccMt>("\"01\"").expect("failed to deserialize");
        assert_eq!(acc_mt.into_inner(), "01");
    }

    #[test]
    fn try_new_with_valid_length_and_digits_should_succeed() {
        let acc_mt = AccMt::try_new("01").expect("failed to create acc_mt");
        assert_eq!(acc_mt.into_inner(), "01");
    }

    #[test]
    fn try_new_with_whitespace_should_fail() {
        let acc_mt = AccMt::try_new("01 ");
        assert!(acc_mt.is_err());
    }

    #[test]
    fn try_new_with_invalid_length_should_fail() {
        // Invalid length of 3
        let acc_mt = AccMt::try_new("011");
        assert!(acc_mt.is_err());
    }

    #[test]
    fn try_new_with_invalid_char_should_fail() {
        let acc_mt = AccMt::try_new("01a");
        assert!(acc_mt.is_err());
    }

    #[test]
    fn try_new_with_zero_should_fail() {
        let acc_mt = AccMt::try_new("00");
        assert!(acc_mt.is_err());
    }
}
