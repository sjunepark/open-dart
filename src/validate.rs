use std::borrow::Cow;
use validator::ValidationError;

fn digit(value: &str) -> Result<(), ValidationError> {
    if value.chars().all(|c| c.is_ascii_digit()) {
        Ok(())
    } else {
        let mut err = ValidationError::new("not_digits");
        err.add_param(Cow::from("value"), &value);
        Err(err)
    }
}

fn string_length(value: &str, min: usize, max: usize) -> Result<(), ValidationError> {
    if value.len() >= min && value.len() <= max {
        Ok(())
    } else {
        let mut err = ValidationError::new("invalid_length");
        err.add_param(Cow::from("value"), &value);
        err.add_param(Cow::from("min"), &min);
        err.add_param(Cow::from("max"), &max);
        Err(err)
    }
}

pub(crate) fn corp_code(value: &str) -> Result<(), ValidationError> {
    digit(value)?;
    string_length(value, 8, 8)?;

    Ok(())
}
