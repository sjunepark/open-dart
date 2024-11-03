use crate::utils::derive_newtype;

derive_newtype!(
    /// 계정ID
    ///
    /// XBRL 표준계정ID
    ///
    /// ※ 표준계정ID가 아닐경우 OpenDart는 이를 ""-표준계정코드 미사용-"" 표시.
    /// 이 경우, None으로 처리.
    #[display("{_0:?}")]
    pub struct AccountId(Option<String>);
);

impl AccountId {
    pub fn new(value: &str) -> Self {
        if value.starts_with("-") {
            Self(None)
        } else {
            Self(Some(value.to_string()))
        }
    }
}

impl From<&str> for AccountId {
    fn from(value: &str) -> Self {
        AccountId::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let account_id = AccountId::new("IFRS_CashAndCashEquivalents");
        assert_eq!(
            account_id.to_string(),
            r#"Some("IFRS_CashAndCashEquivalents")"#
        );
    }
}
