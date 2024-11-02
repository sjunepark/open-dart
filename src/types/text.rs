macro_rules! text {
    ($name:ident, $mock_default:expr) => {
        text!($name, $mock_default, {});
    };
    ($name:ident, $mock_default:expr, {$(#[$doc:meta])*}) => {
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
            pub fn new(value: &str) -> Self {
                Self(value.to_string())
            }

            pub fn into_inner(self) -> String {
                self.0
            }
        }

        #[cfg(test)]
        impl crate::test_utils::MockDefault for $name {
            fn mock_default() -> Self {
                $name::new($mock_default)
            }
        }
    };
}

macro_rules! non_empty_text {
    ($name:ident, $mock_default:expr) => {
        non_empty_text!($name, $mock_default, {});
    };
    ($name:ident, $mock_default:expr, {$(#[$doc:meta])*}) => {
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
                if value.is_empty() {
                    return Err($crate::error::ValidationError {
                        value: value.to_string(),
                        message: format!("Empty value is not allowed for {}", stringify!($name)),
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

// region: Implementations

non_empty_text!(Adres, "경기도 수원시 영통구  삼성로 129 (매탄동)", {
    /// ## 주소
});

non_empty_text!(CeoNm, "한종희", {
    /// ## 대표자명
});

non_empty_text!(CorpName, "삼성전자(주)", {
    /// ## 종목명(법인명)
    ///
    /// 공시대상회사의 종목명(상장사) 또는 법인명(기타법인)
});

non_empty_text!(CorpNameEng, "SAMSUNG ELECTRONICS CO,.LTD", {
    /// ## 영문정식회사명칭
});

non_empty_text!(FaxNo, "031-200-7538", {
    /// ## 팩스번호
});

non_empty_text!(FlrNm, "NH투자증권", {
    /// ## 공시 제출인명
});

non_empty_text!(HmUrl, "www.samsung.com/sec", {
    /// ## 홈페이지
});

text!(IrUrl, "", {
    /// ## IR홈페이지
});

non_empty_text!(PhnNo, "02-2255-0114", {
    /// ## 전화번호
});

non_empty_text!(ReportNm, "[첨부추가]일괄신고추가서류(파생결합증권-주가연계증권)", {
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

text!(RM, "유", {
    /// ### 비고
    /// 조합된 문자로 각각은 아래와 같은 의미가 있음
    /// 조합된 문자로 각각은 아래와 같은 의미가 있음
    /// - 유 : 본 공시사항은 한국거래소 유가증권시장본부 소관임
    /// - 코 : 본 공시사항은 한국거래소 코스닥시장본부 소관임
    /// - 채 : 본 문서는 한국거래소 채권상장법인 공시사항임
    /// - 넥 : 본 문서는 한국거래소 코넥스시장 소관임
    /// - 공 : 본 공시사항은 공정거래위원회 소관임
    /// - 연 : 본 보고서는 연결부분을 포함한 것임
    /// - 정 : 본 보고서 제출 후 정정신고가 있으니 관련 보고서를 참조하시기 바람
    /// - 철 : 본 보고서는 철회(간주)되었으니 관련 철회신고서(철회간주안내)를 참고하시기 바람
});

non_empty_text!(StockName, "삼성전자", {
    /// ## 종목명(상장사) 또는 약식명칭(기타법인)
});

// endregion: Implementations

#[cfg(test)]
mod tests {
    use crate::test_utils::MockDefault;

    non_empty_text!(Name, "Mock Name", {
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
    fn should_not_allow_empty() {
        let _error = Name::try_new("").expect_err("empty name should not be allowed");
    }

    #[test]
    fn mock_default() {
        let name = Name::mock_default();
        assert_eq!(name.into_inner(), "Mock Name");
    }

    #[test]
    fn text_without_doc_comment_should_not_panic() {
        non_empty_text!(MyText, "My Text");

        let my_text = MyText::try_new("My Text").expect("failed to create MyText");
        assert_eq!(my_text.into_inner(), "My Text");
    }
}
