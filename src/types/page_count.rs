use crate::error::{MyValidationError, OpenDartError};
use crate::utils::derive_newtype;

derive_newtype! {
    /// ## 페이지 별 건수
    /// 페이지당 건수(1~100)
    ///
    /// - 기본값 : 10
    /// - 최대값 : 100
    pub struct PageCount(u16);
}

impl PageCount {
    pub fn try_new(page_count: u16) -> Result<Self, OpenDartError> {
        if (1..=100).contains(&page_count) {
            Ok(Self(page_count))
        } else {
            Err(MyValidationError {
                value: page_count.to_string(),
                message: "page_count must be between 1 and 100".to_string(),
            })?
        }
    }

    pub fn into_inner(self) -> u16 {
        self.0
    }
}

#[cfg(test)]
impl crate::test_utils::MockDefault for PageCount {
    fn mock_default() -> Self {
        let page_count = 1;
        PageCount::try_new(page_count)
            .unwrap_or_else(|_| panic!("failed to create PageCount with: {}", page_count))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let page_count = PageCount::try_new(10).expect("failed to create page_count");
        let serialized = serde_json::to_string(&page_count).expect("failed to serialize");
        assert_eq!(serialized, "10");
    }

    #[test]
    fn deserialize() {
        let page_count = serde_json::from_str::<PageCount>("10").expect("failed to deserialize");
        assert_eq!(page_count.into_inner(), 10);
    }

    #[test]
    fn try_new_with_valid_range_should_succeed() {
        let page_count = PageCount::try_new(10).expect("failed to create page_count");
        assert_eq!(page_count.into_inner(), 10);
    }

    #[test]
    fn try_new_with_invalid_range_should_fail() {
        let page_count = PageCount::try_new(0);
        assert!(page_count.is_err());
        let page_count = PageCount::try_new(101);
        assert!(page_count.is_err());
    }
}
