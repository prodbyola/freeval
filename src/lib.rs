use std::collections::HashMap;
use serde::Serialize;

mod validators;

use validators::*;

type ValidatorErrorType = Option<String>;

/// Validation rules used by ```FreeVal``` to validate your input struct.  
pub enum ValidatorRule {
    /// validates length of string
    Length(usize),
    /// validates maximum length of string
    MaxLength(usize),
    /// validates minimum length of string
    MinLength(usize),
    /// validates size of number
    Size(isize),
    /// validates maximum size of number
    MaxSize(isize),
    /// validates minimum size of number 
    MinSize(isize),
    /// validates boolean value
    Bool,
    /// validates password with minimum length
    Password(usize),
    /// validates value is not null
    Required,
    /// validates email address
    Email,
    /// validates range of string length
    LengthRange((isize, isize)),
    /// validates range of int size
    SizeRange((isize, isize)),
}

// field and rules to apply
// type RuleDeclaration = HashMap<String, Vec<RuleType>>;
pub struct RuleDeclaration {
    field: String,
    rules: Vec<RuleType>
}

impl RuleDeclaration {
    /// creates a new rule declaration
    pub fn new(field: &str, rule: ValidatorRule, error: Option<&str>) -> RuleDeclaration {
        let err = RuleDeclaration::create_err(error);
        RuleDeclaration {
            field: field.to_string(),
            rules: vec![RuleType(rule, err)]
        }
    }

    /// Adds a new rule to declaration 
    pub fn insert(&mut self, rule: ValidatorRule, error: Option<&str>) {
        let err = RuleDeclaration::create_err(error);
        self.rules.push(RuleType(rule, err));
    }

    fn create_err(error: Option<&str>) -> ValidatorErrorType {
        let mut err = None;
        if let Some(error) = error {
            err = Some(error.to_string());
        }

        return err
    }
}

// rule and error to be associated
pub struct RuleType(ValidatorRule, ValidatorErrorType);

#[derive(Debug)]
pub struct ValidationResult(bool, HashMap<String, Vec<String>>);

pub struct FreeVal<'a, T: Serialize> {
    pub data: &'a T,
    pub declarations: Vec<RuleDeclaration>,
    
}

impl<'a, T: Serialize> FreeVal<'a, T> {
    pub fn new(data: &'a T, declarations: Vec<RuleDeclaration>) -> FreeVal<'a, T> {
        FreeVal { data, declarations }
    }

    pub fn validate(&self) -> ValidationResult {
        let mut result = ValidationResult(true, HashMap::new());

        if let Ok(serde_json::Value::Object(map)) = serde_json::to_value(self.data) {
            // iterate of keys/values of validator data...
            for (key, value) in &map {
                // ...then iterate over rule declarations to get field's rules
                for decl in &self.declarations {
                    if &decl.field == key {
                        for rule_type in &decl.rules {
                            let mut _inner_result = InnerValidationResult(false, String::new());
    
                            let rule = &rule_type.0;
                            let error = &rule_type.1;
                            let val = value.clone();
                            
                            match rule {
                                ValidatorRule::Length(rule) => _inner_result = length(key, &rule, val, LengthType::Exact),
                                ValidatorRule::MaxLength(rule) => _inner_result = length(key, &rule, val, LengthType::Max),
                                ValidatorRule::MinLength(rule) => _inner_result = length(key, &rule, val, LengthType::Min),
                                ValidatorRule::Size(rule) => _inner_result = size(key, &rule, val, LengthType::Exact),
                                ValidatorRule::MaxSize(rule) => _inner_result = size(key, &rule, val, LengthType::Max),
                                ValidatorRule::MinSize(rule) => _inner_result = size(key, &rule, val, LengthType::Min),
                                ValidatorRule::Bool => _inner_result = check_bool(key, val),
                                ValidatorRule::Password(min_len) => _inner_result = password(key, val, *min_len),
                                ValidatorRule::Required => _inner_result = required(key, val),
                                ValidatorRule::Email => _inner_result = email(key, val),
                                ValidatorRule::LengthRange((min,max)) => _inner_result = range(key, val, min, max, RangeType::Length),
                                ValidatorRule::SizeRange((min, max)) => _inner_result = range(key, val, min, max, RangeType::Size),
                            }
    
                            let InnerValidationResult(status, default_err) = _inner_result;
                            if !status {
                                // Initialize field errors if it does not exist.
                                if let None = result.1.get(key) {
                                    result.1.insert(key.to_string(), Vec::new());
                                }
    
                                if let Some(error_list) = result.1.get(key) {
                                    let errors = self.add_error(error, default_err, error_list);
                                    result.1.insert(key.to_string(), errors);
                                }
                                
                                result.0 = status;
                            }
                
                        }
                    
                    }
                }
            }
        }

        return result;
    }

    fn add_error(&self, defined_err: &ValidatorErrorType, default_err: String, error_list: &Vec<String>) -> Vec<String> {
        let mut error = default_err;

        if let Some(err) = defined_err {
            error = err.to_string();
        }

        let mut errors = error_list.clone().to_vec();
        errors.push(error);

        return errors;
    }
}

#[derive(Serialize)]
struct DemoStruct {
    name: &'static str,
    city: &'static str,
    age: u8,
    bio: Option<String>,
    allow: bool,
    password: &'static str,
    email: &'static str,
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_validator() {
        use super::*;

        let demo = DemoStruct {
            name: "Olamide",
            city: "Nigeria",
            age: 36,
            bio: None,
            allow: true,
            password: "WhatAPass@003",
            email: "myemail@gmailcom"
        };

        // declare validation rules for any field you wish to validate
        let name_rule = RuleDeclaration::new("name", ValidatorRule::Length(12), None);
        let age_rule = RuleDeclaration::new("age", ValidatorRule::Size(18), None);

        let mut bio_rule = RuleDeclaration::new("bio", ValidatorRule::Required, None);
        bio_rule.insert(ValidatorRule::MinLength(12), Some("Bio is too short!")); // We can add more validation rules to a single field

        let allow_rule = RuleDeclaration::new("allow", ValidatorRule::Bool, None);
        let pass_rule = RuleDeclaration::new("password", ValidatorRule::Password(8), Some("Password is incorrect"));
        let email_rule = RuleDeclaration::new("email", ValidatorRule::Email, None);

        // create your validator with declarations
        let val = FreeVal::new(
            &demo,
            vec![name_rule, age_rule, bio_rule, allow_rule, pass_rule, email_rule],
        );

        let ValidationResult(status, _) = val.validate();
        // println!("errors: {:?}", errors);
        
        assert_eq!(status, false)
    }
}
