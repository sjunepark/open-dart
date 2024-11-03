use crate::error::{OpenDartError, ValidationError};
use crate::utils::derive_newtype;

derive_newtype! {
    /// ## API 인증키
    ///
    /// 발급받은 인증키(40자리)
    pub struct CrtfcKey(String);
}

impl CrtfcKey {
    pub fn try_new(value: &str) -> Result<Self, OpenDartError> {
        if value.len() == 40 {
            Ok(Self(value.to_string()))
        } else {
            Err(ValidationError {
                value: value.to_string(),
                message: "crtfc_key must be 40 characters".to_string(),
            })?
        }
    }
}

impl Default for CrtfcKey {
    fn default() -> Self {
        let key = std::env::var("OPEN_DART_API_KEY")
            .expect("OPEN_DART_API_KEY must be set as an environment variable");
        CrtfcKey::try_new(&key).expect("OPEN_DART_API_KEY must be 40 characters")
    }
}

#[cfg(test)]
mod crtfc_key_tests {
    use super::*;

    #[test]
    fn crtfc_key_with_invalid_length() {
        let crtfc_key = CrtfcKey::try_new("1234567890");
        assert!(crtfc_key.is_err());
    }

    #[test]
    fn crtfc_key_with_valid_length() {
        let crtfc_key = CrtfcKey::try_new("1234567890123456789012345678901234567890");
        assert!(crtfc_key.is_ok());
    }
}
