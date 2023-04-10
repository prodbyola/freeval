use serde_json::Value;

pub enum LengthType {
    Exact,
    Max,
    Min
}

impl LengthType {
    pub fn to_string(&self) -> &str {
        match self {
            LengthType::Exact => "exactly",
            LengthType::Max => "maximum of",
            LengthType::Min => "minimum of",
        }
    }
}

/// Validates length of strings or any type has ```len``` method. This is only suitable for strings at the moment.
/// 
/// Panics of ```value``` type does not have ```len``` method.
/// 
/// Returns (ValidationStatus, DefaultValidationError)
pub fn length(field: &str, rule: &usize, value: Value, check_type: LengthType) -> (bool, String) {
    let v: String = serde_json::from_value(value).expect("failed to extract result");
    let err = format!("'{}' field must be {} {} characters.", field, check_type.to_string(), &rule);
    let vlen = &v.len(); // length of value
    let mut cond = vlen == rule;

    match check_type {
        LengthType::Max => cond = vlen >= rule,
        LengthType::Min => cond = vlen <= rule,
        _ => {},
    }

    (cond, err)
}

// Find a way to unify this function and the above
pub fn size(field: &str, rule: &usize, value: Value, check_type: LengthType) -> (bool, String) {
    let v: usize = serde_json::from_value(value).expect("failed to extract result");
    let err = format!("'{}' field must be {} {}.", field, check_type.to_string(), &rule);
    
    let vlen = &v; // length of value
    let mut cond = vlen == rule;

    match check_type {
        LengthType::Max => cond = vlen >= rule,
        LengthType::Min => cond = vlen <= rule,
        _ => {},
    }

    (cond, err)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_length(){
        use super::*;

        let rule = 32;
        let (len_status, _) = length("name", &rule, Value::from("Olamide"), LengthType::Min);
        let (size_status, size_err) = size("age", &rule, Value::from(44), LengthType::Min);

        println!("err: {}", size_err);
        assert_eq!(len_status, true);
        assert_eq!(size_status, false);
    }
}
