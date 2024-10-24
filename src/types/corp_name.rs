use crate::assert_impl_commons_without_default;
use crate::error::{OpenDartError, ValidationError};

use derive_more::{AsRef, Display, From, Into};
use serde::{Deserialize, Serialize};

/// ### 종목명(법인명)
/// 공시대상회사의 종목명(상장사) 또는 법인명(기타법인)
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
pub struct CorpName(String);
assert_impl_commons_without_default!(CorpName);

impl CorpName {
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
use crate::test_utils::MockDefault;
#[cfg(test)]
impl MockDefault for CorpName {
    fn mock_default() -> Self {
        let name = "NH투자증권".to_string();
        CorpName::try_new(&name)
            .unwrap_or_else(|_| panic!("failed to create CorpName with: {}", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let corp_name = CorpName::try_new("NH투자증권").expect("failed to create corp_name");
        let serialized = serde_json::to_string(&corp_name).expect("failed to serialize");
        assert_eq!(serialized, "\"NH투자증권\"");
    }

    #[test]
    fn deserialize() {
        let corp_name =
            serde_json::from_str::<CorpName>("\"NH투자증권\"").expect("failed to deserialize");
        assert_eq!(corp_name.into_inner(), "NH투자증권");
    }

    #[test]
    fn try_new_with_empty_string_should_fail() {
        let corp_name = CorpName::try_new("");
        assert!(corp_name.is_err());
    }
}
