use crate::assert_impl_commons_without_default;
use nutype::nutype;
use static_assertions::assert_impl_all;
use std::fmt::Display;

assert_impl_commons_without_default!(PageNo);
assert_impl_all! {PageNo: Copy}

/// ### 페이지 번호
/// 페이지 번호(1~n)
///
/// - 기본값 : 1
#[nutype(
    validate(greater_or_equal = 1),
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
pub struct PageNo(usize);

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

    #[test]
    fn serialize() {
        let page_no = PageNo::try_new(101).expect("failed to create page_no");
        let serialized = serde_json::to_string(&page_no).expect("failed to serialize");
        assert_eq!(serialized, "101");
    }

    #[test]
    fn deserialize() {
        let page_no = serde_json::from_str::<PageNo>("101").expect("failed to deserialize");
        assert_eq!(page_no.into_inner(), 101);
    }

    #[test]
    fn try_new_with_valid_range_should_succeed() {
        let page_no = PageNo::try_new(101).expect("failed to create page_no");
        assert_eq!(page_no.into_inner(), 101);
    }

    #[test]
    fn try_new_with_invalid_range_should_fail() {
        let page_no = PageNo::try_new(0);
        assert!(page_no.is_err());
    }
}
