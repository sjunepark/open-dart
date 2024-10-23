use crate::assert_impl_commons_without_default;
use nutype::nutype;
use std::fmt::Display;

assert_impl_commons_without_default!(CorpName);

/// ### 종목명(법인명)
/// 공시대상회사의 종목명(상장사) 또는 법인명(기타법인)
#[nutype(
    validate(not_empty),
    derive(
        Clone,
        Eq,
        PartialEq,
        Ord,
        PartialOrd,
        Debug,
        Serialize,
        Deserialize,
        AsRef
    )
)]
pub struct CorpName(String);

impl Display for CorpName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
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
        let corp_name =
            CorpName::try_new("NH투자증권".to_string()).expect("failed to create corp_name");
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
        let corp_name = CorpName::try_new("".to_string());
        assert!(corp_name.is_err());
    }
}
