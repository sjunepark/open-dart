use derive_more::Display;
use serde::{Deserialize, Serialize};

use crate::assert_impl_commons_without_default;

assert_impl_commons_without_default!(YesNo);

#[derive(
    Debug,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    // derive_more
    Display,
    // serde
    Serialize,
    Deserialize,
)]
pub enum YesNo {
    Y,
    N,
}

#[cfg(test)]
impl crate::test_utils::MockDefault for YesNo {
    fn mock_default() -> Self {
        Self::Y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let yes_no = YesNo::Y;
        let serialized = serde_json::to_string(&yes_no).expect("Failed to serialize");
        assert_eq!(serialized, r#""Y""#);
    }

    #[test]
    fn deserialize() {
        let deserialized: YesNo = serde_json::from_str(r#""Y""#).expect("Failed to deserialize");
        assert_eq!(deserialized, YesNo::Y);
    }

    #[test]
    fn display() {
        assert_eq!(YesNo::Y.to_string(), "Y");
    }
}
