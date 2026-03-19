use rand::Rng;
use std::io::{self, Write};

const DIFFICULTY_OPTIONS_STR: &str = 
" 1.   Easy, 1 - 50
 2. Medium, 1 - 200
 3.   Hard, 1 - 1000";

fn get_user_input(prompt: &str) -> String {
    print!("\n{}: ", prompt);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    input.trim().to_string()
}

fn generate_rand_num(num_to_guess: &mut i32, rng: &mut rand::rngs::ThreadRng, difficulty: i32) {
    match difficulty {
        1 => *num_to_guess = rng.gen_range(1..=50),
        2 => *num_to_guess = rng.gen_range(1..=200),
        3 => *num_to_guess = rng.gen_range(1..=1000),
        _ => *num_to_guess = rng.gen_range(1..=200),
    }
}

fn main() {
    println!("Number Guessing Game\n");
    let mut total_guesses: i32 = 0;

    println!("\nDifficulty Options:");
    println!("{}", DIFFICULTY_OPTIONS_STR);
    let mut difficulty_choice: i32 = get_user_input("Choose difficulty(1-3)").parse().expect("Please enter a number");

    // rand num
    let mut rng = rand::thread_rng();
    let mut num_to_guess: i32 = 0;
    generate_rand_num(&mut num_to_guess, &mut rng, difficulty_choice);

    loop {
        let user_answer: i32 = get_user_input("Enter Number").parse().expect("Please enter a number");

        // match user_answer {
        //     Ok(user_answer) => {
                
        //     }
        //     Err(_) => {
        //         println!("Invalid input: Not a valid number.");
        //         continue;
        //     }
        // }

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

            // ask to play again
            let play_again_input = get_user_input("Play Again(y/n)?").to_lowercase().chars().next().unwrap_or(' ');

            if play_again_input != 'y' {
                println!("\nThanks For Playing! Goodbye!");
                break;
            }

            // reset game
            println!("\nDifficulty Options:");
            println!("{}", DIFFICULTY_OPTIONS_STR);
            difficulty_choice = get_user_input("Choose difficulty(1-3)").parse().expect("Please enter a number");
            
            generate_rand_num(&mut num_to_guess, &mut rng, difficulty_choice);
            total_guesses = 0;
        }
    }
}