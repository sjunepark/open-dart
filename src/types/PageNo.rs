use crate::assert_impl_commons_without_default;
use nutype::nutype;
use std::fmt::Display;

assert_impl_commons_without_default!(PageNo);

/// ### 고유번호
/// 공시대상회사의 고유번호(8자리)
///     
/// ※ 개발가이드 > 공시정보 > 고유번호 참고
#[nutype(
    validate(greater_or_equal = 1, less_or_equal = 100),
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
pub struct PageNo(u16);

impl Display for PageNo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[cfg(test)]
use crate::test_utils::MockDefault;
#[cfg(test)]
impl MockDefault for PageNo {
    fn mock_default() -> Self {
        let page_no = 1;
        PageNo::try_new(page_no)
            .unwrap_or_else(|_| panic!("failed to create PageNo with: {}", page_no))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Context;

    #[test]
    fn serialize() -> anyhow::Result<()> {
        let page_no = PageNo::try_new(10).context("failed to create page_no")?;
        let serialized = serde_json::to_string(&page_no).context("failed to serialize")?;
        assert_eq!(serialized, "10");
        Ok(())
    }

    #[test]
    fn deserialize() -> anyhow::Result<()> {
        let page_no = serde_json::from_str::<PageNo>("10").context("failed to deserialize")?;
        assert_eq!(page_no.into_inner(), 10);
        Ok(())
    }

    #[test]
    fn try_new_with_valid_range_should_succeed() -> anyhow::Result<()> {
        let page_no = PageNo::try_new(10).context("failed to create page_no")?;
        assert_eq!(page_no.into_inner(), 10);
        Ok(())
    }

    #[test]
    fn try_new_with_invalid_range_should_fail() -> anyhow::Result<()> {
        let page_no = PageNo::try_new(0);
        assert!(page_no.is_err());
        let page_no = PageNo::try_new(101);
        assert!(page_no.is_err());
        Ok(())
    }
}
