use std::io::{self, Write};

const MENU_PROMPT: &str = 
" 1. Celsius to Fahrenheit
 2. Fahrenheit to Celcius
 3. Celcius to Kelvin
 4. Kelvin to Celcius
 5. Fahrenheit to Kelvin
 6. Kelvin to Fahrenheit
 7. Quit Program";

/* 
    All conversion functions
    just making each a function instead of directly
    computing in match expressions.
    More overhead though
*/
fn celcius_to_fahrenheit(value: f32) -> f32 {
    println!("\nConverting {}°C to Fahrenheit:", value);
    value * 9.0 / 5.0 + 32.0
}   

fn fahrenheit_to_celcius(value: f32) -> f32 {
    println!("\nConverting {}°F to Celcius:", value);
    (value - 32.0) * 5.0 / 9.0
}

fn celcius_to_kelvin(value: f32) -> f32 {
    println!("\nConverting {}°C to Kelvin:", value);
    value + 273.15
}

fn kelvin_to_celcius(value: f32) -> f32 {
    println!("\nConverting {}°K to Celcius:", value);
    value - 273.15
}

fn fahrenheit_to_kelvin(value: f32) -> f32 {
    println!("\nConverting {}°F to Kelvin:", value);
    (value - 32.0) * 5.0 / 9.0 + 273.15
}

fn kelvin_to_fahrenheit(value: f32) -> f32 {
    println!("\nConverting {}°K to Farhrenheit:", value);
    (value - 273.15) * 9.0 / 5.0 + 32.0
}

fn get_user_input(prompt: &str) -> String {
    print!("{}: ", prompt);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    input.trim().to_string()
}

fn handle_user_option(user_conv_option: i32, user_conv_value: f32) -> f32 {
    match user_conv_option {
        1 => celcius_to_fahrenheit(user_conv_value),
        2 => fahrenheit_to_celcius(user_conv_value),
        3 => celcius_to_kelvin(user_conv_value),
        4 => kelvin_to_celcius(user_conv_value),
        5 => fahrenheit_to_kelvin(user_conv_value),
        6 => kelvin_to_fahrenheit(user_conv_value),
        _ => panic!("Invalid Option! Try again."),
    }
}

fn main() {
    println!("Temperature Conversion Program\n");

    loop {
        println!("{}: ", MENU_PROMPT);

        let prompt = "\nEnter Option(1-7)";
        let user_conv_option: i32 = get_user_input(prompt).parse().expect("Please enter a number");

        if user_conv_option == 7 {
            println!("\nGoodbye!");
            break;
        }

        if user_conv_option < 1 || user_conv_option > 7 {
            println!("\nInvalid Input! Try Again\n");
            continue;
        }

        let prompt = "Enter Value to Convert";
        let user_conv_value: f32 = get_user_input(prompt).parse().expect("Please enter a number");

        let result: f32 = handle_user_option(user_conv_option, user_conv_value);

        println!("Result: {}°\n", result);
    }
}
