use rand::Rng;
use std::io::{self, Write};

const DIFFICULTY_OPTIONS_STR: &str = 
" 1.   Easy, 1 - 50   (10 guesses) (1x multiplier)
 2. Medium, 1 - 200  (5 guesses)  (2x multiplier)
 3.   Hard, 1 - 1000 (3 guesses)  (8x multiplier)
 4. Insane, 1 - 5000 (1 guess)    (20x multiplier)
 
Enter Number";

enum DifficultyLevels {
    EASY,
    MEDIUM,
    HARD
}

struct GuessingGame {
    difficulty: u16,
    number_to_guess: u16,
    range: (u16, u16),
    max_attempts: u16,
    bet_multiplier: u16,
}

// can be for individual games
struct GamePlayer {
    total_guesses: u16,
    total_bet: u16,
    money_remaining: u16
}

// Will be adding more params for the functions
impl GuessingGame {
    fn get_user_input(&self, prompt: &str) -> String {
        print!("\n{}: ", prompt);
        io::stdout().flush().expect("Failed to flush stdout");
    
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");
        return input.trim().to_string()
    }

    fn generate_random_num_in_range(&mut self, rng: &mut rand::rngs::ThreadRng, difficulty: u16) {
        match difficulty {
            1 => {
                    self.number_to_guess = rng.gen_range(1..=50);
                    self.range = (1, 50);
                    self.max_attempts = 10;
                    self.bet_multiplier = 1;
                },
            2 => {
                    self.number_to_guess = rng.gen_range(1..=200);
                    self.range = (1, 200);
                    self.max_attempts = 5;
                    self.bet_multiplier = 2;
                },
            3 => {
                    self.number_to_guess = rng.gen_range(1..=1000);
                    self.range = (1, 1000);
                    self.max_attempts = 3;
                    self.bet_multiplier = 8;
                },
            4 => {
                    self.number_to_guess = rng.gen_range(1..=5000);
                    self.range = (1, 5000);
                    self.max_attempts = 1;
                    self.bet_multiplier = 20;
                },
            _ => {
                    self.number_to_guess = rng.gen_range(1..=200);
                    self.range = (1, 200);
                    self.max_attempts = 5;
                    self.bet_multiplier = 2;
                },
        }
    }

    fn check_player_guess(&self, player_guess: u16) -> bool {
        if player_guess > self.number_to_guess {
            println!("Your guess is too high!");
        }

        if player_guess < self.number_to_guess {
            println!("Your guess is too low!");
        }

        return false;
    }

    fn reset_game(&mut self, rng: &mut rand::rngs::ThreadRng) {
        self.difficulty = self.get_user_input(DIFFICULTY_OPTIONS_STR).parse().expect("Please enter a number");
        self.generate_random_num_in_range(rng, self.difficulty);
    }
 
    fn ask_play_again(&self) -> bool {
        let play_again_input = self.get_user_input("Play Again(y/n)?").to_lowercase().chars().next().unwrap_or(' ');
        play_again_input == 'y'
    }

    /* Betting related functions :) */
    fn calculate_winnings(&mut self,  total_bet: u16) -> u16 {
        self.bet_multiplier * total_bet
    }
}

impl GamePlayer {
    fn check_guesses_over(&self, max_attempts: u16) -> bool {
        if self.total_guesses >= max_attempts {
            return true;
        }

        return false;
    }

    fn reset(&mut self) {
        self.total_guesses = 0;
        self.total_bet = 0;
    }

    fn check_player_bet_amount(&self) -> bool {
        if self.total_bet > self.money_remaining {
            return false;
        }

        return true;
    }

    fn get_valid_bet(&mut self, gamestate: &mut GuessingGame) {
        loop {
            println!("\nYour Current Money: {}", self.money_remaining);
            self.total_bet = gamestate.get_user_input("Enter bet amount").parse().expect("Please enter a number");
    
            if !self.check_player_bet_amount() {
                println!("\nInvalid bet! You only have {} in your account.", self.money_remaining);
                println!("Please enter a bet amount that doesn't exceed your money.");
                continue;
            }
    
            println!("\nBet accepted! If you win, you earn: {}", gamestate.calculate_winnings(self.total_bet));
            break;
        }
    }

    fn handle_win(&mut self, gamestate: &mut GuessingGame) {
        println!("\nYour guess is correct! You Win!");
        println!("It took {} guesses", self.total_guesses);
        
        let winnings = gamestate.calculate_winnings(self.total_bet);
        self.money_remaining += winnings;
        println!("You earned: {}. Total Money: {}", winnings, self.money_remaining);
    }
 
    fn handle_loss(&mut self, gamestate: &mut GuessingGame) {
        println!("\nYou reached your max guess attempts!");
        println!("The Correct Number was {}", gamestate.number_to_guess);
        
        self.money_remaining -= self.total_bet;
        println!("You lost your bet of {}. Total Money Remaining: {}", self.total_bet, self.money_remaining);
    }
} 

fn play_game_round(gamestate: &mut GuessingGame, player: &mut GamePlayer) -> bool {
    loop {
        let user_answer: u16 = gamestate.get_user_input("Enter Number").parse().expect("Please enter a number");
        player.total_guesses += 1;
 
        if gamestate.check_player_guess(user_answer) {
            return true; 
        }
 
        if player.check_guesses_over(gamestate.max_attempts) {
            return false; 
        }
    }
}
 
fn handle_game_end(gamestate: &mut GuessingGame, player: &mut GamePlayer, rng: &mut rand::rngs::ThreadRng, player_won: bool) -> bool {
    if player_won {
        player.handle_win(gamestate);
    } else {
        player.handle_loss(gamestate);
    }
 
    if !gamestate.ask_play_again() {
        return false;
    }
 
    gamestate.reset_game(rng);
    player.reset();
    player.get_valid_bet(gamestate);
    true
}

fn main() {
    println!("\n------ Number Guessing Game ------");
    let mut gamestate = GuessingGame { difficulty: 1, number_to_guess: 50, range: (1, 50), max_attempts: 5, bet_multiplier: 1 };
    let mut player = GamePlayer { total_guesses: 0, total_bet: 0, money_remaining: 500 }; 

    let mut rng = rand::thread_rng();
    gamestate.reset_game(&mut rng);
    player.get_valid_bet(&mut gamestate);

    loop {
        let player_won = play_game_round(&mut gamestate, &mut player);
        
        if !handle_game_end(&mut gamestate, &mut player, &mut rng, player_won) {
            println!("\nThanks For Playing! Goodbye!");
            break;
        }
    }
}