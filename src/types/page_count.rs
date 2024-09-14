use crate::assert_impl_commons_without_default;
use nutype::nutype;
use static_assertions::assert_impl_all;
use std::fmt::Display;

assert_impl_commons_without_default!(PageCount);
assert_impl_all! {PageCount: Copy}

/// ### 페이지 별 건수
/// 페이지당 건수(1~100)
///
/// - 기본값 : 10
/// - 최대값 : 100
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
        AsRef,
        Copy
    )
)]
pub struct PageCount(u16);

impl Display for PageCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[cfg(test)]
use crate::test_utils::MockDefault;
#[cfg(test)]
impl MockDefault for PageCount {
    fn mock_default() -> Self {
        let page_count = 1;
        PageCount::try_new(page_count)
            .unwrap_or_else(|_| panic!("failed to create PageCount with: {}", page_count))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Context;

    #[test]
    fn serialize() -> anyhow::Result<()> {
        let page_count = PageCount::try_new(10).context("failed to create page_count")?;
        let serialized = serde_json::to_string(&page_count).context("failed to serialize")?;
        assert_eq!(serialized, "10");
        Ok(())
    }

    #[test]
    fn deserialize() -> anyhow::Result<()> {
        let page_count =
            serde_json::from_str::<PageCount>("10").context("failed to deserialize")?;
        assert_eq!(page_count.into_inner(), 10);
        Ok(())
    }

    #[test]
    fn try_new_with_valid_range_should_succeed() -> anyhow::Result<()> {
        let page_count = PageCount::try_new(10).context("failed to create page_count")?;
        assert_eq!(page_count.into_inner(), 10);
        Ok(())
    }

    #[test]
    fn try_new_with_invalid_range_should_fail() -> anyhow::Result<()> {
        let page_count = PageCount::try_new(0);
        assert!(page_count.is_err());
        let page_count = PageCount::try_new(101);
        assert!(page_count.is_err());
        Ok(())
    }
}
