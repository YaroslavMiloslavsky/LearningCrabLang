Your readiness and ambition are exactly what I look for in a rising Rustacean! I love your proposal:

* **No more 3-year-olds:** Agreed! We'll focus on library crates (unless a binary is explicitly needed for demonstration).
* **Tests and Unit Tests:** Absolutely! Every challenge from now on will require robust test coverage.
* **Examples:** We'll weave them in as appropriate, or note where they could be added for further exploration.
* **Fun and Challenging:** This is my specialty! I will craft challenges that push your limits and make you truly appreciate Rust's design.
* **Generics, Traits, Lifetimes:** As agreed, these will be central to all future exercises.

---

Alright, for **Chapter 11: Automated Tests**, coupled with continuous practice on generics, traits, and lifetimes, I present:

### **Sensei's Dire Challenge: The Universal Data Validator & Sanitizer Library**

You are tasked with building a highly flexible and testable Rust library for validating and sanitizing various types of user input or data records. This library should be generic, use traits for extensible validation rules, and handle lifetimes correctly for borrowed data. It will be a **library crate**.

**Core Requirements:**

1.  **Project Setup (Library Crate):**
    * Create a new library project: `cargo new --lib data_validator`
    * Organize your code into appropriate modules (e.g., `src/lib.rs`, `src/validation_rules.rs`, `src/sanitization_rules.rs`, `src/validator.rs`).

2.  **Generic Data Representation:**
    * Create a generic struct `ValidatedData<'a, T>` that will hold the processed data.
    * It should have a field `value: &'a T`. This signifies that it borrows the data it's validating.
    * Implement `Debug` and `PartialEq` for `ValidatedData`.

3.  **Validation Trait (`ValidatorRule<T>`):**
    * Define a public trait `ValidatorRule<T>`.
    * It should have one method: `validate(&self, data: &T) -> Result<(), String>`.
    * Implementations should return `Ok(())` if the data passes validation, or `Err(String)` with an error message if it fails.
    * **Generics & Lifetimes:** If a validator needs to hold a reference (e.g., a regex pattern), ensure it uses lifetimes correctly.

4.  **Sanitization Trait (`SanitizerRule<T>`):**
    * Define a public trait `SanitizerRule<T>`.
    * It should have one method: `sanitize(&self, data: T) -> T`.
    * Implementations should take ownership of the data, perform a transformation, and return the transformed (sanitized) data.
    * **Generics:** This trait should be generic over the type `T`.

5.  **Concrete Rule Implementations (at least 2 for each):**
    * **For `ValidatorRule<T>`:**
        * `MinLengthValidator`: Implements `ValidatorRule<&str>`. Validates that a string slice has a minimum length.
        * `EmailFormatValidator`: Implements `ValidatorRule<&str>`. Validates if a string slice is a simple email format (e.g., contains '@' and '.'). (No need for complex regex, just basic checks).
        * `PositiveNumberValidator`: Implements `ValidatorRule<i32>`. Validates that an `i32` is positive.
    * **For `SanitizerRule<T>`:**
        * `TrimWhitespaceSanitizer`: Implements `SanitizerRule<String>`. Trims leading/trailing whitespace from a `String`.
        * `ToLowerCaseSanitizer`: Implements `SanitizerRule<String>`. Converts a `String` to lowercase.

6.  **The Core Processor (`DataProcessor<'a, T>`):**
    * Create a public generic struct `DataProcessor<'a, T>`.
    * It should have fields:
        * `validators: Vec<Box<dyn ValidatorRule<T> + 'a>>` (a collection of trait objects for validation).
        * `sanitizers: Vec<Box<dyn SanitizerRule<T>>>` (a collection of trait objects for sanitization).
    * Implement `DataProcessor::new(...)` to construct an instance, allowing arbitrary validators and sanitizers to be passed.
    * Implement a method `process(&self, data: &'a T) -> Result<ValidatedData<'a, T>, String>`.
    * This method should:
        1.  **Sanitize:** Apply all `sanitizers` *sequentially*. Note: Since `sanitize` takes `T` by value, you'll need to carefully manage cloning if `T` is not `Copy` and you want to apply multiple sanitizers, or consider if `sanitize` should take `&mut T` (a design choice you can make and justify). For simplicity, you can make `SanitizerRule` return `T` and clone the input `data` once at the beginning of the `process` method before passing it to the first sanitizer. Subsequent sanitizers then take the output of the previous one.
        2.  **Validate:** Apply all `validators` to the (potentially) sanitized data. If *any* validator returns an `Err`, immediately return that `Err` from `process`.
        3.  If all validations pass, return `Ok(ValidatedData { value: data })`.

7.  **Automated Tests (for `data_validator` crate):**
    * Write **unit tests** for each `ValidatorRule` implementation (e.g., testing `MinLengthValidator` with strings that are too short, just right, and too long).
    * Write **unit tests** for each `SanitizerRule` implementation (e.g., testing `TrimWhitespaceSanitizer` with various whitespace scenarios).
    * Write **integration tests** for the `DataProcessor::process` method, testing:
        * Successful processing with valid data and multiple rules.
        * Failure scenarios where validation rules are violated.
        * How sanitization impacts subsequent validation.
    * Aim for good test coverage.

8.  **`main.rs` in a separate *binary* crate (for demonstration):**
    * Create a separate binary project `cargo new app` in the same workspace as `data_validator` (or just in the same parent directory if not using a workspace for simplicity).
    * In `app/src/main.rs`, add `data_validator` as a dependency in `Cargo.toml`.
    * Demonstrate how to use your `data_validator` library:
        * Create instances of your rules.
        * Construct a `DataProcessor`.
        * Call `process` with various inputs (strings, numbers).
        * Use `match` to handle the `Result` returned by `process`.

**Conceptual Questions (to answer in a separate section after your code):**

1.  **Static vs. Dynamic Dispatch (Again, but Deeper):**
    * You used `Vec<Box<dyn ValidatorRule<T> + 'a>>` and `Vec<Box<dyn SanitizerRule<T>>>`. Why did you choose trait objects (`Box<dyn Trait>`) here instead of using a generic type parameter with trait bounds (like `P: ValidatorRule<T>`) for the `DataProcessor`'s rules? What specific flexibility does this design provide?
    * When would you prefer the static dispatch approach (generics with trait bounds) over dynamic dispatch (trait objects) in a different scenario within a library like this?

2.  **Lifetimes in `ValidatedData<'a, T>` and `DataProcessor<'a, T>`:**
    * Explain the purpose of the `'a` lifetime in `ValidatedData<'a, T>` and `DataProcessor<'a, T>`. What memory safety guarantee does it provide?
    * If you were to store `String`s *owned* by `ValidatedData` (i.e., `value: T` where `T` is `String`), how would the lifetime annotation change, and why?

3.  **Trait Design: `sanitize(&self, data: T) -> T` vs. `sanitize(&mut self, data: &mut T)`:**
    * You chose `sanitize(&self, data: T) -> T` for `SanitizerRule`. Discuss the implications of this choice regarding ownership and data copying/cloning, especially when applying multiple sanitizers sequentially.
    * How would the design change if you chose `sanitize(&mut self, data: &mut T)` instead? What would be the trade-offs (e.g., performance, flexibility, adherence to Copy/Clone)?

This challenge is a true test of your Rust fundamentals. Embrace the compiler errors, as they are your guide to understanding ownership and borrowing. May your tests be green and your data always valid!