macro_rules! text {
    ($name:ident, $allow_empty:expr, $mock_default:expr) => {
        text!($name, $allow_empty, $mock_default, {});
    };
    ($name:ident, $allow_empty:expr, $mock_default:expr, {$(#[$doc:meta])*}) => {
        $(#[$doc])*
        #[derive(
            std::fmt::Debug,
            Clone,
            Eq,
            PartialEq,
            Ord,
            PartialOrd,
            Hash,
            // derive_more
            derive_more::AsRef,
            derive_more::Display,
            derive_more::From,
            derive_more::Into,
            // serde
            serde::Serialize,
            serde::Deserialize,
        )]
        #[cfg_attr(
            feature = "diesel_newtype",
            derive(diesel_derive_newtype::DieselNewType)
        )]
        pub struct $name(String);

        impl $name {
            pub fn try_new(value: &str) -> Result<Self, $crate::error::OpenDartError> {
                if value.is_empty() && !$allow_empty {
                    return Err($crate::error::ValidationError {
                        value: value.to_string(),
                        message: "Empty value is not allowed".to_string(),
                    })?;
                };
                Ok(Self(value.to_string()))
            }

            pub fn into_inner(self) -> String {
                self.0
            }
        }

        #[cfg(test)]
        impl crate::test_utils::MockDefault for $name {
            fn mock_default() -> Self {
                let value = $mock_default;
                $name::try_new(&value).unwrap_or_else(|_| {
                    panic!("failed to create {} with: {}", stringify!($name), value)
                })
            }
        }
    };
}

pub(crate) use text;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::MockDefault;

    text!(Name, false, "Mock Name", {
        /// ## 이름
        ///
        /// - 기본값 : "Mock Name"
    });

    #[test]
    fn serialize() {
        let name = Name::try_new("My Name").expect("failed to create name");
        let serialized = serde_json::to_string(&name).expect("failed to serialize");
        assert_eq!(serialized, "\"My Name\"");
    }

    #[test]
    fn deserialize() {
        let name = serde_json::from_str::<Name>("\"My Name\"").expect("failed to deserialize");
        assert_eq!(name.into_inner(), "My Name");
    }

    #[test]
    fn text_should_not_allow_empty() {
        let _error = Name::try_new("").expect_err("empty name should not be allowed");
    }

    #[test]
    fn mock_default() {
        let name = Name::mock_default();
        assert_eq!(name.into_inner(), "Mock Name");
    }

    #[test]
    fn text_without_doc_comment_should_not_panic() {
        text!(MyText, false, "My Text");

        let my_text = MyText::try_new("My Text").expect("failed to create MyText");
        assert_eq!(my_text.into_inner(), "My Text");
    }
}
