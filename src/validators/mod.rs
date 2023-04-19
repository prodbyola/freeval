use std::fmt::{Display, Debug};

use regex::Regex;
use serde::de::DeserializeOwned;
use serde_json::Value;
pub enum LengthType {
    Exact,
    Max,
    Min,
}

pub struct InnerValidationResult(pub bool, pub String);

impl LengthType {
    pub fn to_string(&self) -> &str {
        match self {
            LengthType::Exact => "exactly",
            LengthType::Max => "maximum of",
            LengthType::Min => "minimum of",
        }
    }
}

pub enum RangeType {
    Size,
    Length,
}

impl RangeType {
    pub fn to_string(&self) -> &str {
        match self {
            RangeType::Size => "size",
            RangeType::Length => "length",
        }
    }
}

/// checks the type of length to be validated
fn check_len<T: PartialEq + PartialOrd>(rule: &T, vlen: &T, length_type: LengthType) -> bool {
    let cond;

    match length_type {
        LengthType::Max => cond = rule >= vlen,
        LengthType::Min => cond = rule <= vlen,
        LengthType::Exact => cond = rule == vlen
    }

    return cond;
}

/// deserializes a value
fn extract_value<T: DeserializeOwned + 'static>(value: Value) -> T {
    let d: T = serde_json::from_value(value).expect("failed to extract result");
    d
}

/// Validates length of strings or any type has ```len``` method. This is most suitable for strings at the moment.
pub fn length(
    field: &str,
    rule: &usize,
    value: Value,
    length_type: LengthType,
) -> InnerValidationResult {
    let err = format!(
        "'{}' field must be {} {} characters.",
        field,
        length_type.to_string(),
        &rule
    );

    if value.is_null() {
        return InnerValidationResult(false, err);
    }

    let v: String = extract_value(value);

    let vlen = &v.len(); // length of value
    let cond = check_len(rule, vlen, length_type);

    InnerValidationResult(cond, err)
}

/// Validates size of an integer
pub fn size(
    field: &str,
    rule: &isize,
    value: Value,
    length_type: LengthType,
) -> InnerValidationResult {
    let err = format!(
        "'{}' field must be {} {}.",
        field,
        length_type.to_string(),
        &rule
    );
    if value.is_null() {
        return InnerValidationResult(false, err);
    }

    let v: isize = extract_value(value);

    let vlen = &v; // length of value
    let cond = check_len(rule, vlen, length_type);

    InnerValidationResult(cond, err)
}

/// checks if required field is not null
pub fn required(field: &str, value: Value) -> InnerValidationResult {
    let err = format!("'{}' field cannot be null.", field);
    InnerValidationResult(!value.is_null(), err)
}

/// checks if a boolean condition is satified
pub fn check_bool(field: &str, value: Value) -> InnerValidationResult {
    let v: bool = extract_value(value);
    let err = format!("'{}' field's condition must be satified.", field);
    InnerValidationResult(v, err)
}

/// validate password
pub fn password(field: &str, value: Value, len: usize) -> InnerValidationResult {
    let err = format!("'{}' field must contain at least one uppercase letter, one lowercase letter, one digit and one special character and must be at least {} chars long.", field, &len);
    if value.is_null() {
        return InnerValidationResult(false, err);
    }

    let v: String = extract_value(value);

    let mut has_whitespace = false;
    let mut has_upper = false;
    let mut has_lower = false;
    let mut has_digit = false;
    let mut has_special_char = false;

    for c in v.chars() {
        has_whitespace |= c.is_whitespace();
        has_lower |= c.is_lowercase();
        has_upper |= c.is_uppercase();
        has_digit |= c.is_digit(10);
        has_special_char |= !c.is_ascii_alphanumeric()
    }

    let cond = !has_whitespace
        && has_upper
        && has_lower
        && has_digit
        && has_special_char
        && v.len() >= len;
    InnerValidationResult(cond, err)
}

/// Validates email address
pub fn email(field: &str, value: Value) -> InnerValidationResult {
    let err = format!("'{}' field must be a valid email address", field);
    if value.is_null() {
        return InnerValidationResult(false, err);
    }

    let v: String = extract_value(value);
    let re = Regex::new(r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$").unwrap();
    InnerValidationResult(re.is_match(&v), err)
}

/// Validates whether the ```length``` of a ```string``` or the ```size``` of an ```int``` is within a specified 
/// range of ```min``` and ```max```.
pub fn range<T>(
    field: &str,
    value: Value,
    min: &T,
    max: &T,
    range_type: RangeType,
) -> InnerValidationResult
where
    T: DeserializeOwned + PartialOrd + Display + 'static + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
{
    let err = format!(
        "{}'s {} must be between {} and {}.",
        field,
        range_type.to_string(),
        min,
        max
    );

    if value.is_null() {
        return InnerValidationResult(false, err);
    }

    let len: T;

    match range_type {
        RangeType::Length => {
            let val: String = extract_value(value);
            let nv = T::try_from(val.len()).unwrap();
            len = nv;
        }
        RangeType::Size => len = extract_value(value),
    }

    let cond = &len > min && &len < max;
    InnerValidationResult(cond, err)
}

pub fn contains(field: &str, rule: &str, value: Value) -> InnerValidationResult {
    let err = format!("'{}' field must contain  '{}'. Please check again.", field, rule);
    if value.is_null() {
        return InnerValidationResult(false, err);
    }

    let v: String = extract_value(value);

    let cond = v.contains(rule);

    InnerValidationResult(cond, err)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_length() {
        use super::*;

        let len_rule = 7;
        let size_rule = -32;
        let InnerValidationResult(len_status, _) =
            length("name", &len_rule, Value::from("Olamide"), LengthType::Min); // length
        let InnerValidationResult(size_status, _) =
            size("age", &size_rule, Value::from(44), LengthType::Max); // size
        let InnerValidationResult(req_status, _) = required("valid", Value::from(Some("yes"))); // required
        let InnerValidationResult(bool_status, _) = check_bool("allow", Value::from(false)); // boolean
        let InnerValidationResult(pass_status, _) =
            password("password", Value::from("MyUniquPas@007"), 8); // password
        let InnerValidationResult(email_status, _) = email("email", Value::from("MyUniquPas@007")); // email

        // range
        let (min, max) = (8,16);
        let InnerValidationResult(rlen_status, _) = range::<i32>("rlen", Value::from("TheRandomString"), &min, &max, RangeType::Length); // length
        let InnerValidationResult(slen_status, _) = range("slen", Value::from(6), &min, &max, RangeType::Size); // size

        let InnerValidationResult(cont_status, _) = contains("contains_field", "nothere", Value::from("I love rust")); // contains

        assert_eq!(len_status, true);
        assert_eq!(size_status, false);
        assert_eq!(req_status, true);
        assert_eq!(bool_status, false);
        assert_eq!(pass_status, true);
        assert_eq!(email_status, false);
        assert_eq!(rlen_status, true);
        assert_eq!(slen_status, false);
        assert_eq!(cont_status, false);
    }
}
