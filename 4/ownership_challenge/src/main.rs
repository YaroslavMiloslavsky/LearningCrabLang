#![warn(clippy::all, clippy::pedantic)]

#[allow(unused)]
fn main() {
    let mut my_string = String::from("Rust is fun");

    // Function 1 -> Challenge Test
    // takes_ownership(my_string);
    // println!("{my_string}");
    // The compile Error I get is 'borrow of moved value: `my_string`'
    // It happens because the function takes_ownership moves the variable my_string and drops it inside the function after the print
    // After that this variable is available no more
    // End of Function 1 -> Challenge Test

    // Function 2 -> Challenge Test
    let l = calculates_length(&my_string);
    println!("length of {my_string} is {l}");
    // This is different because calculates_length borrows my_string, thus not dropping the value at its' end
    // After the call, the borrow checker releases the variable back to its' previous owner, my_string
    // End of Function 2 -> Challenge Test

    // Function 3 -> Challenge Test 1
    modifies_string(&mut my_string);
    println!("{my_string}");
    // End of Function 3 -> Challenge Test 1
    
    // Function 3 -> Challenge Test 2
    // modifies_string(&mut my_string);
    // let a = &my_string;
    // let another_ref = &mut my_string;
    // println!("{a}");
    // println!("{my_string}");
    // End of Function 3 -> Challenge Test 2

    // Function 4 -> Challenge Test
    let fw = first_word(&my_string);
    // my_string.clear();
    // cannot borrow `my_string` as mutable because it is also borrowed as immutable mutable borrow occurs here
    // The compiler know that fw still may be used later, the slice that was returned from first_word, is a pointer to the heap with len that points to my_string
    // The compiler knows that if we clear the heap, there will be still a dangling pointer
    println!("{fw}");
    // End of Function 4 -> Challenge Test

    // Conceptual Question
    // String is allocated on the heap and holds the continues data of the string, while &str points to the specific place in the heap, but is allocated on the stack, and acts as a reference.
    // I would use String for input buffer for example, or a source of truth for some textual data, which will be referenced with &str's
    // I would use &str when a flexibility is needed and when I want to pass the string or part of it to a function
    // End of Conceptual Question
}

// Function 1
#[allow(unused)]
fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

// Function 2
fn calculates_length(s: &String) -> usize {
    println!("Received string: {s}");
    s.len()
}

// Function 3
fn modifies_string(s: &mut String) {
    s.push_str(" ");
    s.push_str("and powerful!");
}

// Function 4
fn first_word(s: &String) -> &str {
    let bytes = s.trim().as_bytes();

    for (i, &character) in bytes.iter().enumerate() {
        if character == b' ' {
            return &s[..i]
        }
    }
    &s
}
