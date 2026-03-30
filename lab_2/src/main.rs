use rand::Rng;
use std::io::{self, Write};


const GAME_MODES_STR: &str = 
" 1.        Forever - Play as long as you want
 2.   Sudden Death - First to win a round
 3.   Best of Five - Play 5 rounds
 4. First to Three - First to 3 wins
 
Enter Number";

#[derive(Debug, PartialEq, Clone, Copy)]
enum Move {
    ROCK,
    PAPER,
    SCISSORS
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum GameMode {
    FOREVER,
    SUDDEN_DEATH,
    BEST_OF_FIVE,
    FIRST_TO_THREE,
}

#[derive(PartialEq)]
enum Outcome {
    WIN,
    LOSE,
    DRAW
}

struct RPSGame {
    player: RPSPlayer,
    computer: RPSPlayer,
    rounds: u16
}

struct RPSPlayer {
    score: u16,
    name: String
}

impl GameMode {
    fn get_mode_input(prompt: &str) -> Option<GameMode> {
        print!("\n{}: ", prompt);
        io::stdout().flush().expect("Failed to flush stdout");
    
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");

        match input.trim().parse::<i32>() {
            Ok(1) => Some(GameMode::FOREVER),
            Ok(2) => Some(GameMode::SUDDEN_DEATH),
            Ok(3) => Some(GameMode::BEST_OF_FIVE),
            Ok(4) => Some(GameMode::FIRST_TO_THREE),
            _ => None
        }
    }
}

impl Move {
    fn get_user_move(prompt: &str) -> Option<Move> {
        print!("\n{}: ", prompt);
        io::stdout().flush().expect("Failed to flush stdout");
    
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");

        match input.trim().to_lowercase().as_str() {
            "rock" | "r" => Some(Move::ROCK),
            "paper" | "p" => Some(Move::PAPER),
            "scissors" | "s" => Some(Move::SCISSORS),
            _ => None
        }
    }

    fn random_move(rng: &mut rand::rngs::ThreadRng) -> Move {
        match rng.gen_range(0..=2) {
            0 => Move::ROCK,
            1 => Move::PAPER,
            _ => Move::SCISSORS
        }
    }

    fn beats(&self, other: &Move) -> Outcome {
        match (self, other) {
            (Move::ROCK, Move::SCISSORS) => Outcome::WIN,
            (Move::PAPER, Move::ROCK) => Outcome::WIN,
            (Move::SCISSORS, Move::PAPER) => Outcome::WIN,
            (a, b) if a == b => Outcome::DRAW,
            _ => Outcome::LOSE
        }
    }
}

impl RPSPlayer {
    fn new(name: &str) -> RPSPlayer {
        RPSPlayer {
            name: name.to_string(),
            score: 0
        }
    }
}

impl RPSGame {
    fn new() -> RPSGame {
        RPSGame {
            player: RPSPlayer::new("Human"),
            computer: RPSPlayer::new("Computer"),
            rounds: 0
        }
    }

    fn get_user_input(&self, prompt: &str) -> String {
        print!("\n{}: ", prompt);
        io::stdout().flush().expect("Failed to flush stdout");
    
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");
        return input.trim().to_string()
    }

    fn print_score(&self) {
        println!("Current Score: {} - {}", self.player.score, self.computer.score);
    }

    fn print_winner(&self) {
        if self.player.score > self.computer.score {
            println!("\n----------------------------\nYou Win! Score: {} - {}\n----------------------------", self.player.score, self.computer.score);
        } else if self.player.score < self.computer.score {
            println!("\n----------------------------\nComputer Wins! Score: {} - {}\n----------------------------", self.computer.score, self.player.score);
        }
    }

    fn play_round(&mut self, mut player_move: Move) {
        let mut rng = rand::thread_rng();
        let mut computer_move = Move::random_move(&mut rng);
        self.rounds += 1;

        println!("{} chose {:?} - {} chose {:?}", self.player.name, player_move, self.computer.name, computer_move);

        match player_move.beats(&computer_move) {
            Outcome::WIN => {
                self.player.score += 1;
                println!("\nYou Win this Round!");
                self.print_score();
            },
            Outcome::LOSE => {
                self.computer.score += 1;
                println!("\nYou Lose this Round!");
                self.print_score();
            }
            Outcome::DRAW => {
                println!("\nYou Draw!");
                self.print_score();
            }
        }
    }

    fn ask_play_again(&self) -> bool {
        let play_again_input = self.get_user_input("Play Again(y/n)?").to_lowercase().chars().next().unwrap_or(' ');
        play_again_input == 'y'
    }

    fn is_game_over(&self, game_mode: GameMode) -> bool {
        match game_mode {
            GameMode::FOREVER => false,
            GameMode::SUDDEN_DEATH => self.player.score > 0 || self.computer.score > 0,
            GameMode::BEST_OF_FIVE => self.rounds >= 5,
            GameMode::FIRST_TO_THREE => self.player.score >= 3 || self.computer.score >= 3
        }
    }

    fn reset_game(&mut self) {
        self.player.score = 0;
        self.computer.score = 0;
        self.rounds = 0;
    }
}

fn main() {
    println!("\n------ Rock Paper Scissors ------");

    loop {
        let mut game_mode = GameMode::get_mode_input(GAME_MODES_STR).expect("Invalid Game Mode");
        println!("\nChosen Game Mode: {:?}", game_mode);
        let mut game = RPSGame::new();

        // rounds in a game
        loop {
            let player_move = Move::get_user_move("Enter Move(rock, paper, scissors)").expect("Invalid Move");
            game.play_round(player_move);

            if game.is_game_over(game_mode) {
                game.print_winner();
                break;
            }
        }

        if !game.ask_play_again() {
            println!("Thanks for Playing! Goodbye!");
            break;
        } else {
            game.reset_game();
        }
    }
}