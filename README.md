## FreeVal
**FreeVal** is a rust crate mostly useful for struct field validations.  
  
But why **FreeVal** when we already have a rust [validator](https://github.com/Keats/validator)?  
Existing rust [validator](https://github.com/Keats/validator) is great for many use cases especially where you have full control over your ```struct``` definitions... i.e, you can define attributes for the ```struct``` and all its members in order to set validation rules. In my case however, I needed to validate data generated from ```protobuf``` messages for a gRPC server. If I make changes to these definitions, they would be overwritten when I edit and regenerate my proto definitions. So I have to find another way to validate the data. **FreeVal** is born to fix the problem.

With **FreeVal**, you don't to define attributes on your struct members. Simple implement (or derive) ```serde::Serialize``` and that's all. FreeVal takes care of the rest.

### Usage
```rust
use freeval::*;

#[derive(serde::Serialize)]
struct RequestData{
    username: String,
    password: String
} 

fn main() {
    let data = RequestData {
        username: &'static str,
        password: &'static str
    };

    // first, declare validation rules for each field you wish to validate with (optional) error message...
    let mut username_rule = declare_rule!("username", ValidatorRule::LengthRange((8, 12)), "username length is too short! Must be between 8 and 12");
    insert_rule!(username_rule, ValidatorRule::Required); // you can insert more rules for a single field. 

    let pass_rule = declare_rule!("password", ValidatorRule::Password(8), "Password unacceptable!"); // another rule for the "password" field.

    //...then create your validator with declared rules
    let validator = freeval!(&data, vec![username_rule, pass_rule]);
    //... and validate 
    let vr = validator.validate();

    if let Err(err) = &vr {
        // err is an HashMap of each field and their validation errors(if any).
        println!("validation errors: {:?}", err);

        // validation errors: 
        // {
        //     "username": ["username length is too short! Must be between 8 and 12"], 
        //     "password": ["Password unacceptable!"]
        // }
    }

    assert!(vr.is_err());
}
```

### Validator Rule(s)
**FreeVal**'s validation rules are declared through ```ValidationRule``` enum (as seen in the example above: ```ValidationRule::Required```). ```ValidationRule``` enum has the following variants:

## Future Validation
* Phone
* Credit Card
* Regex
* MustMatch
