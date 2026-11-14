use std::io::{Write, stdin};

fn main() {
    println!("Please enter two numbers and an operator space-separated, like \"2 2 +\".");
    println!("+ to sum, - to subtract, * to multiply and / to divide. Use Ctrl+C to exit.");
    // println!("Type \"quit\" or \"q\" to exit.");
    loop {
        let mut whole = String::new();
        print!("Input > ");
        flush();
        stdin().read_line(&mut whole).expect("Failed to read line.");
        let whole: Vec<&str> = whole.split(" ").collect();
        if let [first_digit, second_digit, operator] = whole[0..=2] {
            let first_digit: i32 = first_digit.trim().parse().expect("ERR: First digit is NaN");
            let second_digit: i32 = second_digit
                .trim()
                .parse()
                .expect("ERR: Second digit is NaN");
            let operator = operator.trim();

            let result = match operator {
                "+" => sum(first_digit, second_digit),
                "-" => subtract(first_digit, second_digit),
                "*" => multiply(first_digit, second_digit),
                "/" => divide(first_digit, second_digit),
                &_ => panic!("Invalid operator!"),
            };

            println!("Result is: {result}\n");
        } else {
            println!("ERR: Please enter three elements.\n");
        };
    }
}

fn flush() {
    std::io::stdout().flush().expect("Unable to flush output");
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
