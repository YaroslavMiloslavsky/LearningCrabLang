use std::{io, u128};

fn main() {
    println!("Welcome to the fibonacci function!");
    'game_loop: loop {
        let mut user_input_string = String::new();
        let input: u32 = 'prompt_user: loop {
            println!("Please enter a positive non 0 value");
            user_input_string.clear();
            io::stdin()
                .read_line(&mut user_input_string)
                .expect("Couldn't read the input from the user, aborting...");

            match user_input_string.trim().parse() {
                Ok(num) => break 'prompt_user num,
                Err(e) => {
                    println!(
                        "Could not parse the input, please provide a valid non negative value: {e}"
                    );
                    continue 'prompt_user;
                }
            }
        };

        if input == 0 {
            continue 'game_loop;
        }

        let fib = generate_fibonacci(input);
        for number in fib {
            print!("{number} ");
        }

        'replay_loop: loop {
            println!("Do you wish to try again? Y(es)/N(o)");
            user_input_string.clear();
            io::stdin()
                .read_line(&mut user_input_string)
                .expect("Couldn't read the input from user, aborting...");

            user_input_string = user_input_string.trim().to_lowercase();
            if user_input_string == "y" || user_input_string == "yes" {
                continue 'game_loop;
            }
            if user_input_string == "n" || user_input_string == "no" {
                break 'game_loop;
            } else {
                println!("Please provide a valid answer only");
                continue 'replay_loop;
            }
        }
    }
    println!("Goodbye!")
}

fn generate_fibonacci(n: u32) -> Vec<u128> {
    let mut fib_vec: Vec<u128> = vec![];

    if n > 0 {
        fib_vec.push(0);
        if n > 1 {
            fib_vec.push(1);
            'fib_loop: for i in 2..(n as usize) {
                let first = fib_vec[i - 1];
                let second = fib_vec[i - 2];
                let sum: u128 = match first.checked_add(second) {
                    Some(num) => num,
                    None => {
                        println!("Adding {first} and {second} would cause an overflow");
                        break 'fib_loop;
                    }
                };
                fib_vec.push(sum);
            }
        }
    }

    fib_vec
}
