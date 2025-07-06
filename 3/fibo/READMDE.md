# Second challenge
You have a very strong grasp of Chapter 3's concepts. Your ability to explain the "why" behind Rust's design choices (like immutability by default and debug vs. release build behavior) is excellent.

Now, for your next challenge!

Sensei's Second Coding Challenge: Fibonacci Sequence Generator with Control Flow
Your task is to write a Rust program that generates the Fibonacci sequence up to a user-specified number of terms. However, it will include several twists to test your understanding of Chapter 3's concepts.

### Requirements:

User Input for N:

Ask the user how many Fibonacci numbers they want to generate.

Use the robust input parsing and error handling you learned in Chapter 2 (i.e., gracefully handle non-numeric input by prompting again, don't panic!).

Ensure the input is a positive integer. If the user enters 0 or a negative number, prompt them to enter a positive number.

Fibonacci Generation Function:

Create a function generate_fibonacci(n: u32) that takes a u32 representing the number of terms.

This function should return a Vec<u128> (a vector/dynamic array of u128 numbers) containing the sequence. Why u128? Because Fibonacci numbers grow very rapidly, and u128 will allow you to generate a significant number of terms before hitting overflow.

Handle edge cases:

If n is 0, return an empty vector.

If n is 1, return [0].

If n is 2, return [0, 1].

Control Flow and Data Types:

Inside your generate_fibonacci function, use a for loop to iterate and calculate the terms.

Be mindful of integer types for your sequence calculation. If you calculate a + b using smaller integer types, you might hit overflow before converting to u128 for storage. Consider how to avoid this.

Bonus Challenge (if you're up for it): Implement an if expression to check for potential u128 overflow before adding the next term. If adding the next two numbers would exceed u128::MAX, stop generating the sequence at that point and perhaps print a warning, returning the sequence generated so far. (Hint: checked_add method on integer types is very useful here!)

### Main Program Loop:

In main, call your generate_fibonacci function.

Print the generated Fibonacci sequence to the console.

After displaying the sequence, ask the user if they want to generate another sequence (similar to your "play again" logic from the guessing game).

Thought Process Guidance:

Break down the problem into smaller functions.

Think about the return types of your functions.

Prioritize robust user input.

Consider how u128 handles large numbers and potential overflow.

Remember Rust's expressions vs. statements for concise code.

This challenge will test your ability to combine all the concepts from Chapter 3, along with the robust I/O from Chapter 2.

Take your time, experiment, and don't hesitate to consult the Rust book or documentation if you get stuck on a specific syntax or method.

<i><b>May your code compile on the first try!</b></i>