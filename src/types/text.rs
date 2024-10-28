use crate::error::{OpenDartError, ValidationError};
use crate::statics::assert_impl_commons_without_default;
use derive_more::{AsRef, Display, From, Into};
use serde::{Deserialize, Serialize};

assert_impl_commons_without_default!(Text);
#[derive(
    Debug,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    // derive_more
    AsRef,
    Display,
    From,
    Into,
    // serde
    Serialize,
    Deserialize,
)]
#[cfg_attr(
    feature = "diesel_newtype",
    derive(diesel_derive_newtype::DieselNewType)
)]
pub struct Text(String);

impl Text {
    pub fn try_new(value: &str) -> Result<Self, OpenDartError> {
        if value.is_empty() {
            return Err(ValidationError {
                value: value.to_string(),
                message: "empty string is not allowed".to_string(),
            })?;
        };
        Ok(Self(value.to_string()))
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

#[cfg(test)]
impl crate::test_utils::MockDefault for Text {
    fn mock_default() -> Self {
        let name = "NH투자증권".to_string();
        Text::try_new(&name).unwrap_or_else(|_| panic!("failed to create Name with: {}", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let name = Text::try_new("NH투자증권").expect("failed to create name");
        let serialized = serde_json::to_string(&name).expect("failed to serialize");
        assert_eq!(serialized, "\"NH투자증권\"");
    }

    #[test]
    fn deserialize() {
        let name = serde_json::from_str::<Text>("\"NH투자증권\"").expect("failed to deserialize");
        assert_eq!(name.into_inner(), "NH투자증권");
    }

    #[test]
    fn try_new_with_empty_string_should_fail() {
        let name = Text::try_new("");
        assert!(name.is_err());
    }
}
