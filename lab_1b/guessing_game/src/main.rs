use rand::Rng;
use std::io::{self, Write};

fn get_user_input(prompt: &str) -> String {
    print!("\n{}: ", prompt);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    input.trim().to_string()
}

fn generate_rand_num(num_to_guess: &mut i32, rng: &mut rand::rngs::ThreadRng) {
    *num_to_guess = rng.gen_range(1..=100);
}

fn main() {
    println!("Number Guessing Game\n");
    // println!("Difficulty Options:");
    // println!(
    // "1. Easy"    
    // )

    // print!("Choose an option: ");
    // io::stdout().flush().expect("Failed to flush stdout");

    // let mut input = String::new();
    // io::stdin().read_line(&mut input).expect("Failed to read");
    // input.trim().to_string()

    let mut total_guesses: i32 = 0;

    // rand num
    let mut rng = rand::thread_rng();
    let mut num_to_guess: i32 = 0;
    generate_rand_num(&mut num_to_guess, &mut rng);
    println!("{}", num_to_guess);

    loop {
        let user_answer: i32 = get_user_input("Enter Number").parse().expect("Please enter a number");
        total_guesses+=1;

        if user_answer > num_to_guess {
            println!("Your guess is too high!");
            continue;
        }

        if user_answer < num_to_guess {
            println!("Your guess is too low!");
            continue;
        }

        if user_answer == num_to_guess {
            println!("\nYour guess is correct! You Win!");
            println!("It took {} guesses", total_guesses);
            generate_rand_num(&mut num_to_guess, &mut rng);
            total_guesses = 0;

            let play_again_input = get_user_input("Play Again(y/n)?: ").to_lowercase().chars().next().unwrap_or(' ');

            if play_again_input != 'y' {
                println!("\nThanks For Playing! Goodbye!");
                break;
            }
        }
    }
}
