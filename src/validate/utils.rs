use std::borrow::Cow;
use validator::ValidationError;

pub(crate) fn is_digit(value: &str) -> Result<(), ValidationError> {
    if value.chars().all(|c| c.is_ascii_digit()) {
        Ok(())
    } else {
        let mut err = ValidationError::new("not_digits");
        err.add_param(Cow::from("value"), &value);
        Err(err)
    }
}

pub(crate) fn check_string_length(
    value: &str,
    min: usize,
    max: usize,
) -> Result<(), ValidationError> {
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

pub(crate) fn contains(available: &[&str], got: &str) -> Result<(), ValidationError> {
    if available.contains(&got) {
        Ok(())
    } else {
        let mut err = ValidationError::new("not_in_list");
        err.add_param(Cow::from("available"), &available);
        err.add_param(Cow::from("needle"), &got);
        Err(err)
    }
}
