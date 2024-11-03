use crate::error::{OpenDartError, ValidationError};
use crate::utils::derive_newtype;

derive_newtype! {
    /// ## 총 페이지 수
    pub struct PageNo(u64);
}

impl PageNo {
    pub fn try_new(page_no: u64) -> Result<Self, OpenDartError> {
        if page_no == 0 {
            Err(ValidationError {
                value: page_no.to_string(),
                message: "page_no must be greater than 0".to_string(),
            })?
        } else {
            Ok(Self(page_no))
        }
    }

    pub fn into_inner(self) -> u64 {
        self.0
    }
}

// region: Implementation

/// ## 총 건수
/// 총 페이지 수
pub type TotalCount = PageNo;

/// ## 총 페이지 수
pub type TotalPage = PageNo;

// endregion: Implementation

#[cfg(test)]
impl crate::test_utils::MockDefault for PageNo {
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
