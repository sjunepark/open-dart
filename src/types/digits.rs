macro_rules! digits {
    ($name:ident, $allow_empty:expr, $mock_default:expr, $digits:expr) => {
        digits!($name, $allow_empty, $mock_default, $digits, {});
    };
    ($name:ident, $allow_empty:expr, $mock_default:expr, $digits:expr, {$(#[$doc:meta])*}) => {
        $crate::utils::derive_newtype! {
            $(#[$doc])*
            pub struct $name(String);
        }

        impl $name {
            pub fn try_new(value: &str) -> Result<Self, $crate::error::OpenDartError> {
                if value.is_empty() {
                    if $allow_empty {
                        return Ok(Self(value.to_string()));
                    } else {
                        return Err($crate::error::ValidationError {
                            value: value.to_string(),
                            message: format!("Empty value is not allowed for {}", stringify!($name)),
                        })?;
                    }
                };

                if value.len() == $digits && value.chars().all(|c| c.is_ascii_digit()) {
                    Ok(Self(value.to_string()))
                } else {
                    Err($crate::error::ValidationError {
                        value: value.to_string(),
                        message: concat!(stringify!($name), " must be ", $digits, " digits").to_string(),
                    })?
                }
            }

            pub fn into_inner(self) -> String {
                self.0
            }
        }

        impl TryFrom<&str> for $name {
            type Error = $crate::error::OpenDartError;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::try_new(value)
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

// region: Implementations

digits!(BizrNo, false, "1248100998", 10, {
    /// ## 사업자등록번호
    ///
    /// 10자리
});

digits!(CorpCode, false, "00126380", 8, {
    /// ## 고유번호
    ///
    /// 공시대상회사의 고유번호(8자리)
    ///
    /// ※ 개발가이드 > 공시정보 > 고유번호 참고
});

digits!(IndutyCode, false, "264", 3, {
    /// ## 업종코드
    ///
    /// 3자리
});

digits!(JurirNo, false, "1301110006246", 13, {
    /// ## 법인등록번호
    ///
    /// 13자리
});

digits!(RceptNo, false, "20200117000486", 14, {
    /// ### 접수번호
    /// 접수번호(14자리)
    ///
    /// ※ 공시뷰어 연결에 이용예시
    /// - PC용 : https://dart.fss.or.kr/dsaf001/main.do?rcpNo=접수번호
});

digits!(StockCode, true, "005930", 6, {
    /// ## 주식코드
    ///
    /// 6자리
});

// endregion: Implementations

#[cfg(test)]
mod tests {
    use crate::test_utils::MockDefault;

    digits!(FiveDigit, false, "54321", 5, {
        /// ## 이름
        ///
        /// - 기본값 : "Mock Name"
    });

    #[test]
    fn serialize() {
        let name = FiveDigit::try_new("12345").expect("failed to create name");
        let serialized = serde_json::to_string(&name).expect("failed to serialize");
        assert_eq!(serialized, "\"12345\"");
    }

    #[test]
    fn deserialize() {
        let name = serde_json::from_str::<FiveDigit>("\"12345\"").expect("failed to deserialize");
        assert_eq!(name.into_inner(), "12345");
    }

    #[test]
    fn mock_default() {
        let name = FiveDigit::mock_default();
        assert_eq!(name.into_inner(), "54321");
    }

    #[test]
    fn text_without_doc_comment_should_not_panic() {
        digits!(MyDigit, false, "54321", 5);

        let my_digit = MyDigit::try_new("12345").expect("failed to create MyText");
        assert_eq!(my_digit.into_inner(), "12345");
    }

    #[test]
    fn try_new_should_not_allow_empty() {
        let _error = FiveDigit::try_new("").expect_err("empty name should not be allowed");
    }

    #[test]
    fn try_new_with_zeros_should_work() {
        let name = FiveDigit::try_new("00000").expect("failed to create name");
        assert_eq!(name.into_inner(), "00000");
    }

    #[test]
    fn try_new_with_whitespace_should_fail() {
        let name = FiveDigit::try_new("12345 ");
        assert!(name.is_err());
    }

    #[test]
    fn try_new_with_invalid_length_should_fail() {
        let name = FiveDigit::try_new("1234");
        assert!(name.is_err());
    }

    #[test]
    fn try_new_with_invalid_char_should_fail() {
        let name = FiveDigit::try_new("1234a");
        assert!(name.is_err());
    }

    digits!(EmptyDefault, true, "012345", 6);

    impl Default for EmptyDefault {
        fn default() -> Self {
            EmptyDefault::try_new("").expect("failed to create empty EmptyDefault")
        }
    }

    #[test]
    fn should_be_able_to_create_default_empty_stock_code() {
        let stock_code = EmptyDefault::default();
        assert_eq!(stock_code.into_inner(), "");
    }
}
