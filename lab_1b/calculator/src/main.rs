use std::io::{self, Write};

fn add(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

fn subtract(num1: i32, num2: i32) -> i32 {
    num1 - num2
}

fn multiply(num1: i32, num2: i32) -> i32 {
    num1 * num2
}

fn divide(num1: i32, num2: i32) -> i32 {
    if num2 == 0 {
        panic!("Division by Zero Error!")
    }

    num1 / num2
}

fn handle_operation(operation: char, num1: i32, num2: i32) -> i32 {
    match operation {
        '+' => add(num1, num2),
        '-' => subtract(num1, num2),
        '*' => multiply(num1, num2),
        '/' => divide(num1, num2),
          _ => panic!("Invalid Operator! Try again."),
    }
}

fn get_user_input(enter_prompt: &str) -> String {
    print!("{}: ", enter_prompt);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    input.trim().to_string()
}

fn main() {
    // Number 1
    let prompt = "\nEnter a number";
    let num1_input: i32 = get_user_input(&prompt).parse().expect("Please enter a number");

    // Operation
    let prompt = "Enter an operation(+, -, *, /)";
    let operation_input: char = get_user_input(prompt).parse().expect("Please enter an operation(+, -, *, /)");

    // Number 2
    let prompt = "Enter a number";
    let num2_input: i32 = get_user_input(&prompt).parse().expect("Please enter a number");

    let result: i32 = handle_operation(operation_input, num1_input, num2_input);

    // output
    println!("\n{} {} {} = {}", num1_input, operation_input, num2_input, result);
}