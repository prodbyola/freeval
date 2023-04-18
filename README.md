## FreeVal
**FreeVal** is a rust crate mostly useful for backend validation of form inputs (or field validations).  
  
But why **FreeVal** when we already have a rust [validator](https://github.com/Keats/validator)?  
Existing rust [validator](https://github.com/Keats/validator) is great for many use cases especially where you have full control over your ```struct``` definitions... i.e, you can define attributes for the ```struct``` and all its members in order to set validation rules. In my case however, I needed to validate data generated from ```protobuf``` messages for a gRPC server. If I make changes to these definitions, they would be overwritten when I edit and regenerate my proto definitions. So I have to find another way to validate the data. FreeVal is born to fix the problem.

## Future Validation
* Phone
* Credit Card
* Regex
* MustMatch