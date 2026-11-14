use std::{
    io::{Write, stdin},
    num::ParseIntError,
};

fn main() {
    println!("Please enter two numbers and an operator space-separated, like \"2 2 +\".");
    println!("Basic arithmetics only (+-*/) Use Ctrl+C to exit.");
    // println!("Type \"quit\" or \"q\" to exit.");
    loop {
        let mut whole = String::new();
        print!("Input > ");
        flush();
        stdin().read_line(&mut whole).expect("Failed to read line.");
        let whole: Vec<&str> = whole.trim().split(" ").collect();
        let [first_digit, second_digit, operator] = *whole else {
            println!("ERROR: Please input 3 elements");
            continue;
        };
        let first_digit = match parse_to_i32(first_digit) {
            Ok(n) => n,
            Err(_) => {
                println!("ERROR: First digit is an invalid number!\n");
                continue;
            }
        };
        let second_digit: i32 = match parse_to_i32(second_digit) {
            Ok(n) => n,
            Err(_) => {
                println!("ERROR: Second digit is an invalid number!\n");
                continue;
            }
        };
        let operator = operator.trim();

        let result = match operator {
            "+" => sum(first_digit, second_digit),
            "-" => subtract(first_digit, second_digit),
            "*" => multiply(first_digit, second_digit),
            "/" => divide(first_digit, second_digit),
            &_ => panic!("Invalid operator!"),
        };

        println!("Result is: {result}\n");
    }
}

fn flush() {
    std::io::stdout().flush().expect("Unable to flush output");
}

fn parse_to_i32(s: &str) -> Result<i32, ParseIntError> {
    s.trim().parse()
}

fn sum(digit1: i32, digit2: i32) -> i32 {
    digit1 + digit2
}

fn subtract(digit1: i32, digit2: i32) -> i32 {
    digit1 - digit2
}

fn multiply(digit1: i32, digit2: i32) -> i32 {
    digit1 * digit2
}

fn divide(digit1: i32, digit2: i32) -> i32 {
    digit1 / digit2
}
