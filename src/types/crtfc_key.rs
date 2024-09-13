use crate::assert_impl_commons;
use nutype::nutype;
use std::fmt::{Display, Formatter};

assert_impl_commons!(CrtfcKey);

/// ### API 인증키
/// 발급받은 인증키(40자리)
#[nutype(
    validate(len_char_min = 40, len_char_max = 40),
    derive(
        Clone,
        Eq,
        PartialEq,
        Ord,
        PartialOrd,
        Debug,
        Serialize,
        Deserialize,
        AsRef,
        TryFrom
    )
)]
pub struct CrtfcKey(String);

impl Default for CrtfcKey {
    fn default() -> Self {
        let key = std::env::var("OPEN_DART_API_KEY")
            .expect("OPEN_DART_API_KEY must be set as an environment variable");
        CrtfcKey::try_new(key).expect("OPEN_DART_API_KEY must be 40 characters")
    }
}

impl Display for CrtfcKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[cfg(test)]
mod crtfc_key_tests {
    use super::*;

    #[test]
    fn crtfc_key_with_invalid_length() {
        let crtfc_key = CrtfcKey::try_new("1234567890".to_string());
        assert!(crtfc_key.is_err());
    }

    #[test]
    fn crtfc_key_with_valid_length() {
        let crtfc_key = CrtfcKey::try_new("1234567890123456789012345678901234567890".to_string());
    }
}
