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

// region: Implementations
text!(CorpName, false, "삼성전자(주)", {
    /// ## 종목명(법인명)
    ///
    /// 공시대상회사의 종목명(상장사) 또는 법인명(기타법인)
});

text!(CorpNameEng, false, "SAMSUNG ELECTRONICS CO,.LTD", {
    /// ## 영문정식회사명칭
});

text!(ReportNm, false, "[첨부추가]일괄신고추가서류(파생결합증권-주가연계증권)", {
    /// ## 보고서명
    /// 공시구분+보고서명+기타정보
    ///
    /// - 기재정정: 본 보고서명으로 이미 제출된 보고서의 기재내용이 변경되어 제출된 것임
    /// - 첨부정정: 본 보고서명으로 이미 제출된 보고서의 첨부내용이 변경되어 제출된 것임
    /// - 첨부추가: 본 보고서명으로 이미 제출된 보고서의 첨부서류가 추가되어 제출된 것임
    /// - 변경등록: 본 보고서명으로 이미 제출된 보고서의 유동화계획이 변경되어 제출된 것임
    /// - 연장결정: 본 보고서명으로 이미 제출된 보고서의 신탁계약이 연장되어 제출된 것임
    /// - 발행조건확정: 본 보고서명으로 이미 제출된 보고서의 유가증권 발행조건이 확정되어 제출된 것임
    /// - 정정명령부과: 본 보고서에 대하여 금융감독원이 정정명령을 부과한 것임
    /// - 정정제출요구: 본 보고서에 대하여 금융감독원이 정정제출요구을 부과한 것임
});

text!(StockName, false, "삼성전자", {
    /// ## 종목명(상장사) 또는 약식명칭(기타법인)
});

text!(CeoNm, false, "한종희", {
    /// ## 대표자명
});

text!(Adres, false, "경기도 수원시 영통구  삼성로 129 (매탄동)", {
    /// ## 주소
});

text!(HmUrl, false, "www.samsung.com/sec", {
    /// ## 홈페이지
});

text!(IrUrl, true, "", {
    /// ## IR홈페이지
});

text!(PhnNo, false, "02-2255-0114", {
    /// ## 전화번호
});

text!(FaxNo, false, "031-200-7538", {
    /// ## 팩스번호
});
// endregion: Implementations

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
