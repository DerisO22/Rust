use std::io;

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
    num1 / num2
}

fn main() {
    // Number 1
    let mut input = String::new();
    println!("Enter a number: ");
    io::stdin().read_line(&mut input).expect("Failed to read");

    let num1_input: i32 = input.trim().parse().expect("Please enter a number");

    // Operation
    let mut input = String::new();
    println!("Enter an operation(+, -, *, /)");
    io::stdin().read_line(&mut input).expect("Failed to read");

    let operation_input: char = input.trim().parse().expect("Please enter an operation(+, -, *, /)");
    println!("Operation: {}", operation_input);

    // Number 2
    let mut input = String::new();
    println!("Enter a number: ");
    io::stdin().read_line(&mut input).expect("Failed to read");

    let num2_input: i32 = input.trim().parse().expect("Please enter a number");

    let result: i32 = add(num1_input, num2_input);

    println!("Result = {}", result);
}