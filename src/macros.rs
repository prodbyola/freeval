#[macro_export]
macro_rules! freeval {
    ($data:expr, $rules:expr) => {
        FreeVal::new($data, $rules)
    };
}

#[macro_export]
macro_rules! declare_rule {
    ($field:expr, $rule:expr) => {
        RuleDeclaration::new($field, $rule, None)
    };
    ($field:expr, $rule:expr, $err:expr) => {
        RuleDeclaration::new($field, $rule, Option::from($err))
    }
}

#[macro_export]
macro_rules! insert_rule {
    ($decl:expr, $rule:expr) => {
        $decl.insert($rule, None)
    };
    ($decl:expr, $rule:expr, $err:expr) => {
        $decl.insert($rule, Option::from($err))
    };
}