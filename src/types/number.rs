use serde::{Deserialize, Deserializer, Serializer};
use std::fmt::Display;
use std::str::FromStr;

macro_rules! number {
    ($name:ident) => {
        number!($name, {});
    };
    ($name:ident, {$(#[$attr:meta])*}) => {
        $crate::utils::derive_newtype! {
            $(#[$attr])*
            pub struct $name(
                #[serde(serialize_with = "crate::types::serialize_number_as_string")]
                #[serde(deserialize_with = "serde_aux::prelude::deserialize_number_from_string")]
                i64
            );
        }

        impl $name {
            pub fn new(value: i64) -> Self {
                Self(value)
            }
        }

        #[cfg(test)]
        impl crate::test_utils::MockDefault for $name {
            fn mock_default() -> Self {
                $name::new(1)
            }
        }
    };
}

macro_rules! optional_number {
    ($name:ident) => {
        optional_number!($name, {});
    };
    ($name:ident, {$(#[$attr:meta])*}) => {
        $crate::utils::derive_newtype! {
            $(#[$attr])*
            #[display("{_0:?}")]
            pub struct $name(
                #[serde(serialize_with = "crate::types::serialize_optional_number_as_string")]
                #[serde(deserialize_with = "crate::types::deserialize_optional_number_from_string")]
                Option<i64>
            );
        }

        impl $name {
            pub fn new(value: Option<i64>) -> Self {
                Self(value)
            }
        }

        #[cfg(test)]
        impl crate::test_utils::MockDefault for $name {
            fn mock_default() -> Self {
                $name::new(Some(1))
            }
        }
    };
}

pub(crate) fn serialize_number_as_string<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: std::fmt::Display,
{
    serializer.serialize_str(&value.to_string())
}

pub(crate) fn serialize_optional_number_as_string<S, T>(
    value: &Option<T>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: std::fmt::Display,
{
    match value {
        Some(v) => serializer.serialize_str(&v.to_string()),
        None => serializer.serialize_none(),
    }
}

pub(crate) fn deserialize_optional_number_from_string<'de, T, D>(
    deserializer: D,
) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr + Deserialize<'de>,
    <T as FromStr>::Err: Display,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrInt<T> {
        String(String),
        Number(T),
        None,
    }

    match StringOrInt::<T>::deserialize(deserializer)? {
        StringOrInt::String(s) => {
            if s.is_empty() {
                Ok(None)
            } else {
                s.parse::<T>().map(Some).map_err(serde::de::Error::custom)
            }
        }
        StringOrInt::Number(i) => Ok(Some(i)),
        StringOrInt::None => Ok(None),
    }
}

// region: Implementations

optional_number!(BfefrmtrmAmount, {
    /// ## 전전기금액
    ///
    /// 9,999,999,999(※ 사업보고서의 경우에만 출력)
});

optional_number!(FrmtrmAmount, {
    /// ## 전기금액
    ///
    /// 9,999,999,999
});

optional_number!(FrmtrmAddAmount, {
    /// ## 전기누적금액
    ///
    /// 9,999,999,999
});

optional_number!(FrmtrmQAmount, {
    /// ## 전기금액(분/반기)
    ///
    /// 9,999,999,999 ※ 분/반기 보고서이면서 (포괄)손익계산서 일 경우 \[3개월] 금액
});

number!(Ord, {
    /// ## 계정과목 정렬순서
});

number!(ThstrmAmount, {
    /// ## 당기금액
    ///
    /// 9,999,999,999 ※ 분/반기 보고서이면서 (포괄)손익계산서 일 경우 \[3개월] 금액
});

optional_number!(ThstrmAddAmount, {
    /// ## 당기누적금액
    ///
    /// 9,999,999,999
});

// endregion: Implementations

#[cfg(test)]
mod tests {
    number!(TestAmount, {
        /// ## Test Amount
    });

    #[test]
    fn serialize() {
        let amount = TestAmount::new(1);
        let serialized = serde_json::to_string(&amount).expect("Failed to serialize");
        assert_eq!(serialized, r#""1""#);
    }

    #[test]
    fn deserialize() {
        let amount = TestAmount::new(1);
        let deserialized: TestAmount =
            serde_json::from_str(r#""1""#).expect("Failed to deserialize");
        assert_eq!(deserialized, amount);
    }

    #[test]
    fn empty_string_should_deserialize_none() {
        optional_number!(TestAmount, {
            /// ## Test Amount
        });
        let amount = TestAmount::new(None);
        let deserialized: TestAmount =
            serde_json::from_str(r#""""#).expect("Failed to deserialize");
        assert_eq!(deserialized, amount);
    }
}
