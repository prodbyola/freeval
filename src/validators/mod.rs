use regex::Regex;
use serde::de::DeserializeOwned;
use serde_json::Value;

pub enum LengthType {
    Exact,
    Max,
    Min
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

/// checks the type of length to be validated
fn check_len(rule: &usize, vlen: &usize, check_type: LengthType) -> bool {
    let mut cond = vlen == rule;

    match check_type {
        LengthType::Max => cond = vlen >= rule,
        LengthType::Min => cond = vlen <= rule,
        _ => {},
    }

    return cond
}

/// deserializes a value
fn extract_value<T: DeserializeOwned + 'static>(value: Value) -> T {
    let d:T = serde_json::from_value(value).expect("failed to extract result");
    d
}

/// Validates length of strings or any type has ```len``` method. This is most suitable for strings at the moment.
pub fn length(field: &str, rule: &usize, value: Value, check_type: LengthType) -> InnerValidationResult {
    let err = format!("'{}' field must be {} {} characters.", field, check_type.to_string(), &rule);

    if value.is_null() {
        return InnerValidationResult(false, err)
    }

    let v: String = extract_value(value);

    let vlen = &v.len(); // length of value
    let cond = check_len(rule, vlen, check_type);

    InnerValidationResult(cond, err)
}

/// Validates size of an integer
pub fn size(field: &str, rule: &usize, value: Value, check_type: LengthType) -> InnerValidationResult {
    let err = format!("'{}' field must be {} {}.", field, check_type.to_string(), &rule);
    if value.is_null() {
        return InnerValidationResult(false, err)
    }

    let v: usize = extract_value(value);
    
    let vlen = &v; // length of value
    let cond = check_len(rule, vlen, check_type);

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
        return InnerValidationResult(false, err)
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

    let cond = !has_whitespace && has_upper && has_lower && has_digit && has_special_char && v.len() >= len;
    InnerValidationResult(cond, err)
}

/// Validates email address
pub fn email(field: &str, value: Value) -> InnerValidationResult {
    let err = format!("'{}' field must be a valid email address", field);
    if value.is_null() {
        return InnerValidationResult(false, err)
    }

    let v: String = extract_value(value);
    let re = Regex::new(r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$").unwrap();
    InnerValidationResult(re.is_match(&v), err)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_length(){
        use super::*;

        let rule = 32;
        let InnerValidationResult(len_status, _) = length("name", &rule, Value::from("Olamide"), LengthType::Min); // length
        let InnerValidationResult(size_status, _) = size("age", &rule, Value::from(44), LengthType::Min); // size
        let InnerValidationResult(req_status, _) = required("valid", Value::from(Some("yes"))); // required
        let InnerValidationResult(bool_status, _) = check_bool("allow", Value::from(false)); // boolean
        let InnerValidationResult(pass_status, _) = password("password", Value::from("MyUniquPas@007"), 8); // password
        let InnerValidationResult(email_status, _) = email("email", Value::from("MyUniquPas@007")); // email

        assert_eq!(len_status, true);
        assert_eq!(size_status, false);
        assert_eq!(req_status, true);
        assert_eq!(bool_status, false);
        assert_eq!(pass_status, true);
        assert_eq!(email_status, false);
    }
}
