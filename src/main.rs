use std::io::{Write, stdin};

fn main() {
    println!(
        "Welcome to the calculator. Please enter a number, followed by any of the 4 standard math operations:"
    );
    println!("+ to sum, - to subtract, * to multiply and / to divide. Use Ctrl+C to exit.");
    // Type \"quit\" or \"q\" to exit.
    loop {
        let mut first_digit = String::from("");
        let mut second_digit = String::from("");
        let mut operator = String::from("");

        readnum_cycle("Enter first number: ", &mut first_digit);
        readnum_cycle("Enter operator (+-*/) ", &mut operator);
        readnum_cycle("Enter second number: ", &mut second_digit);

        let first_digit: i32 = first_digit
            .trim()
            .parse()
            .expect("Failed to parse first digit: NaN");
        let second_digit: i32 = second_digit
            .trim()
            .parse()
            .expect("Failed to parse second digit: NaN");
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

fn readnum_cycle(text: &str, buff: &mut String) {
    print!("{text}");
    flush();
    stdin().read_line(buff).expect("Failed to read line.");
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
