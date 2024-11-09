use super::utils;
use chrono::NaiveDate;
use std::borrow::Cow;
use validator::ValidationError;

macro_rules! optional_string {
    ($raw_fn:ident) => {
        paste::paste! {
            pub(crate) fn [<optional_ $raw_fn>](value: &Option<String>) -> Result<(), ValidationError> {
                if let Some(value) = value {
                    $raw_fn(&value)
                } else {
                    Ok(())
                }
            }
        }
    };
}

pub(crate) fn yyyymmdd(value: &str) -> Result<(), ValidationError> {
    const FORMAT: &str = "%Y%m%d";

    let _ = NaiveDate::parse_from_str(value, FORMAT).map_err(|_| {
        let mut err = ValidationError::new("invalid_date");
        err.add_param(Cow::from("value"), &value);
        err.add_param(Cow::from("format"), &FORMAT);
        err
    })?;

    Ok(())
}
optional_string!(yyyymmdd);

pub(crate) fn bsns_year(value: &str) -> Result<(), ValidationError> {
    utils::is_digit(value)?;
    utils::check_string_length(value, 4, 4)?;

    // Parse to string
    let year = value
        .parse::<u64>()
        .expect("The value should have been validated as a digit before parsing");

    // Check if the year is in the range 2000..=2100
    if (2000..=2100).contains(&year) {
        Ok(())
    } else {
        let mut err = ValidationError::new("invalid_year");
        err.add_param(Cow::from("value"), &value);
        Err(err)
    }
}

pub(crate) fn corp_cls(value: &str) -> Result<(), ValidationError> {
    const CORP_CLS: [&str; 4] = ["Y", "K", "N", "E"];
    utils::contains(&CORP_CLS, value)?;

    Ok(())
}
optional_string!(corp_cls);

pub(crate) fn corp_code(value: &str) -> Result<(), ValidationError> {
    utils::is_digit(value)?;
    utils::check_string_length(value, 8, 8)?;

    Ok(())
}
optional_string!(corp_code);

pub(crate) fn fs_div(value: &str) -> Result<(), ValidationError> {
    const FS_DIVS: [&str; 2] = ["CFS", "OFS"];
    utils::contains(&FS_DIVS, value)?;

    Ok(())
}

pub(crate) fn pblntf_ty(value: &str) -> Result<(), ValidationError> {
    const PBLNTF_TYS: [&str; 10] = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J"];
    utils::contains(&PBLNTF_TYS, value)?;

    Ok(())
}
optional_string!(pblntf_ty);

pub(crate) fn pblntf_detail_ty(value: &str) -> Result<(), ValidationError> {
    const PBLNTF_DETAIL_TYS: [&str; 60] = [
        "A001", "A002", "A003", "A004", "A005", "B001", "B002", "B003", "C001", "C002", "C003",
        "C004", "C005", "C006", "C007", "C008", "C009", "C010", "C011", "D001", "D002", "D003",
        "D004", "D005", "E001", "E002", "E003", "E004", "E005", "E006", "E007", "E008", "E009",
        "F001", "F002", "F003", "F004", "F005", "G001", "G002", "G003", "H001", "H002", "H003",
        "H004", "H005", "H006", "I001", "I002", "I003", "I004", "I005", "I006", "J001", "J002",
        "J004", "J005", "J006", "J008", "J009",
    ];
    utils::contains(&PBLNTF_DETAIL_TYS, value)?;

    Ok(())
}
optional_string!(pblntf_detail_ty);

pub(crate) fn reprt_code(value: &str) -> Result<(), ValidationError> {
    const REPRT_CODES: [&str; 4] = ["11013", "11012", "11014", "11011"];
    utils::contains(&REPRT_CODES, value)?;

    Ok(())
}

pub(crate) fn sort(value: &str) -> Result<(), ValidationError> {
    const SORTS: [&str; 3] = ["date", "crp", "rpt"];
    utils::contains(&SORTS, value)?;

    Ok(())
}
optional_string!(sort);

pub(crate) fn sort_mth(value: &str) -> Result<(), ValidationError> {
    const SORT_MTHS: [&str; 2] = ["asc", "desc"];
    utils::contains(&SORT_MTHS, value)?;

    Ok(())
}
optional_string!(sort_mth);

pub(crate) fn yes_no(value: &str) -> Result<(), ValidationError> {
    const YES_NO: [&str; 2] = ["Y", "N"];
    utils::contains(&YES_NO, value)?;

    Ok(())
}
optional_string!(yes_no);
