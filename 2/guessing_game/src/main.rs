use std::io;

use rand::Rng;

fn main() {
    loop {
        game_loop();
        loop {
            println!("Do you wish to play again? Y(es)/N(o)");
            let mut user_input = String::new();
            io::stdin()
                .read_line(&mut user_input)
                .expect("Could not read user input");

            user_input = user_input.trim().to_lowercase();

            if user_input == "y" || user_input == "yes" {
                break;
            }
            if user_input == "n" || user_input == "no" {
                return;
            }
            println!("Only Y/N or Yes/No are valid options, try again.")
        }
    }
}

fn game_loop() {
    println!("Please enter a number between 0 and 100");

    let random_num = generate_random_int();
    let mut guesses_left = 5;

    while guesses_left > 0 {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Could read the input!");

        let guess: usize = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Couldn't parse the number: {e}");
                continue;
            }
        };

        if equal(&guess, &random_num) {
            return;
        } else {
            println!("Try again!\n");
            guesses_left -= 1;
        }
    }
    println!("No more guesses left");
    println!("The secret number is: {random_num}");
}

fn equal(guess: &usize, actual: &usize) -> bool {
    match guess.cmp(actual) {
        std::cmp::Ordering::Equal => {
            println!("Correct!");
            true
        }
        std::cmp::Ordering::Greater => {
            println!("Too big!");
            false
        }
        std::cmp::Ordering::Less => {
            println!("Too small!");
            false
        }
    }
}

fn generate_random_int() -> usize {
    let mut rng = rand::rng();
    rng.random_range(0..=100)
}
