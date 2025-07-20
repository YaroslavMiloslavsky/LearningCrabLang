use data_validator::DataProcessor;
use data_validator::EmailFormatValidator;
use data_validator::MinLengthValidator;

use data_validator::ToLowerCaseSanitizer;
use data_validator::TrimWhitespaceSanitizer;

pub fn create_processor_email_max_len_20() -> DataProcessor<String>{
    let email_val = Box::new(EmailFormatValidator::new());
    let min_val = Box::new(MinLengthValidator::new(20));

    let trim_san = Box::new(TrimWhitespaceSanitizer::new());
    let low_san = Box::new(ToLowerCaseSanitizer::new());

   DataProcessor::new(vec![email_val, min_val], vec![trim_san, low_san])
}
fn main() {
    let p = create_processor_email_max_len_20();
    match p.process(&mut String::from("example@example.com")) {
        Ok(_) => println!("All Good"),
        Err(e) => println!("Error: {e:?}"),
    }
}

/*
Conceptual Questions
Static vs. Dynamic Dispatch
Please mind you we did not cover dyn and boxes yet, I learnt them myself for this task.
I chose box with dyn for runtime polymorphism, using generics and phantom data did not work because no info on runtime for compiler.

I would prefer static dispatch when designing a function for a very specific limited functions which I know what the type of will be, this provides better performance but costs in flexibility.

*/

/*
 Lifetimes in ValidatedData<'a, T> and DataProcessor<'a, T>
 I did without, there was no need as DataProcessor did not hold any reference by me design.
 Have it needed one, I would use 'a.
 It promises that the DataProcessor will live as long as the held value.
I did, look at the code.
*/

/*
Trait Design
I changed the design after testing few possibilities, it compiled, and the code uses as little clone as possible (non)
*/