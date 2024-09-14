use crate::assert_impl_commons_without_default;
use nutype::nutype;
use std::fmt::Display;

assert_impl_commons_without_default!(CorpCode);

/// ### 고유번호
/// 공시대상회사의 고유번호(8자리)
///     
/// ※ 개발가이드 > 공시정보 > 고유번호 참고
#[nutype(
    validate(len_char_min = 8, len_char_max = 8, predicate = is_digits),
    derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize, AsRef)
)]
pub struct CorpCode(String);

impl Display for CorpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[cfg(test)]
use crate::test_utils::MockDefault;
#[cfg(test)]
impl MockDefault for CorpCode {
    fn mock_default() -> Self {
        let corp_code: String = String::from("00120182");
        CorpCode::try_new(&corp_code)
            .unwrap_or_else(|_| panic!("failed to create CorpCode with: {}", corp_code))
    }
}

fn is_digits(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Context;

    #[test]
    fn serialize() -> anyhow::Result<()> {
        let corp_code =
            CorpCode::try_new("00120182".to_string()).context("failed to create corp_code")?;
        let serialized = serde_json::to_string(&corp_code).context("failed to serialize")?;
        assert_eq!(serialized, "\"00120182\"");
        Ok(())
    }

    #[test]
    fn deserialize() -> anyhow::Result<()> {
        let corp_code =
            serde_json::from_str::<CorpCode>("\"00120182\"").context("failed to deserialize")?;
        assert_eq!(corp_code.into_inner(), "00120182");
        Ok(())
    }

    #[test]
    fn try_new_with_valid_length_and_digits_should_succeed() -> anyhow::Result<()> {
        let corp_code =
            CorpCode::try_new("00120182".to_string()).context("failed to create corp_code")?;
        assert_eq!(corp_code.into_inner(), "00120182");
        Ok(())
    }

    #[test]
    fn try_new_with_whitespace_should_fail() -> anyhow::Result<()> {
        let corp_code = CorpCode::try_new("00120182 ".to_string());
        assert!(corp_code.is_err());
        Ok(())
    }

    #[test]
    fn try_new_with_invalid_length_should_fail() -> anyhow::Result<()> {
        // Invalid length of 7
        let corp_code = CorpCode::try_new("0012018".to_string());
        assert!(corp_code.is_err());
        Ok(())
    }

    #[test]
    fn try_new_with_invalid_char_should_fail() -> anyhow::Result<()> {
        let corp_code = CorpCode::try_new("0012018a".to_string());
        assert!(corp_code.is_err());
        Ok(())
    }
}
