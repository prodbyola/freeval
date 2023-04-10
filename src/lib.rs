use std::collections::HashMap;
use serde::Serialize;

mod validators;

use validators::{length, size, LengthType};

type ValidatorErrorType = Option<String>;

pub enum ValidatorRule {
    Length(usize),
    MaxLength(usize),
    MinLength(usize),
    Size(usize),
    MaxSize(usize),
    MinSize(usize),
    Bool(bool),
    Password,
    Required,
}

// field and rules to apply
type RuleDeclaration = HashMap<String, Vec<RuleType>>;

// rule and error to be associated
pub struct RuleType(ValidatorRule, ValidatorErrorType);

#[derive(Debug)]
pub struct ValidationResult(bool, HashMap<String, Vec<String>>);

pub struct Validator<'a, T: Serialize> {
    pub data: &'a T,
    pub declarations: RuleDeclaration,
    
}

impl<'a, T: Serialize> Validator<'a, T> {
    pub fn new(data: &'a T, declarations: RuleDeclaration) -> Validator<'a, T> {
        Validator { data, declarations }
    }

    pub fn validate(&self) -> ValidationResult {
        let mut result = ValidationResult(false, HashMap::new());

        if let Ok(serde_json::Value::Object(map)) = serde_json::to_value(self.data) {
            // iterate of keys/values of validator data...
            for (key, value) in &map {
                // ...then get user defined RuleTypes for each key (if the rule is declared with a valid key)
                if let Some(rules) = self.declarations.get(key) {
                    // iterate and validate rule 
                    for rule_type in rules {
                        let (mut status, mut default_err) = (false, String::new());

                        let rule = &rule_type.0;
                        let error = &rule_type.1;
                        
                        match rule {
                            ValidatorRule::Length(rule) => (status, default_err) = length(key, &rule, value.clone(), LengthType::Exact),
                            ValidatorRule::MaxLength(rule) => (status, default_err) = length(key, &rule, value.clone(), LengthType::Max),
                            ValidatorRule::MinLength(rule) => (status, default_err) = length(key, &rule, value.clone(), LengthType::Min),
                            ValidatorRule::Size(rule) => (status, default_err) = size(key, &rule, value.clone(), LengthType::Exact),
                            ValidatorRule::MaxSize(rule) => (status, default_err) = size(key, &rule, value.clone(), LengthType::Max),
                            ValidatorRule::MinSize(rule) => (status, default_err) = size(key, &rule, value.clone(), LengthType::Min),
                            ValidatorRule::Bool(_) => {}
                            ValidatorRule::Password => {}
                            ValidatorRule::Required => {}
                        }

                        if !status {
                            // Initialize errors if it does not exist.
                            if let None = result.1.get(key) {
                                result.1.insert(key.to_string(), Vec::new());
                            }

                            if let Some(error_list) = result.1.get(key) {
                                let errors = self.add_error(error, default_err, error_list);
                                result.1.insert(key.to_string(), errors);
                            }
                        }
            
                        result.0 = status;
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

        // let mut err_map = HashMap::new();
        // err_map.insert(field.to_string(), errors);

        return errors;
    }
}

#[derive(Serialize)]
struct DemoStruct {
    name: &'static str,
    city: &'static str,
    age: u8,
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_validator() {
        use super::*;

        let demo = DemoStruct {
            name: "Olamide",
            city: "Nigeria",
            age: 36
        };

        // declare your validation rules
        let mut declarations = HashMap::new();

        declarations.insert("name".to_string(), vec![RuleType(ValidatorRule::Length(12), None)]);
        declarations.insert("city".to_string(), vec![RuleType(ValidatorRule::MinLength(18), None)]);
        declarations.insert("age".to_string(), vec![RuleType(ValidatorRule::Size(18), None)]);

        // create your validator with declarations
        let validator = Validator::new(
            &demo,
            declarations,
        );

        let ValidationResult(status, errors) = validator.validate();
        println!("errors: {:?}", errors);
        
        assert_eq!(status, false)
    }
}
