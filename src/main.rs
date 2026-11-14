use std::{
    io::{Write, stdin},
    num::ParseIntError,
};

fn main() {
    println!("Please enter two numbers and an operator space-separated, like \"2 2 +\".");
    println!("Basic arithmetics only (+-*/). Use Ctrl+C to exit.");
    // println!("Type \"quit\" or \"q\" to exit.");
    loop {
        let mut whole = String::new();
        print!("Input > ");
        flush();
        stdin().read_line(&mut whole).expect("Failed to read line.");
        let whole: Vec<&str> = whole.trim().split(" ").collect();
        let [first_digit, second_digit, operator] = *whole else {
            println!("ERROR: Please input 3 elements.");
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

        let math = Maths::new(first_digit, second_digit);
        let result = match operator {
            "+" => math.sum(),
            "-" => math.subtract(),
            "*" => math.multiply(),
            "/" => math.divide(),
            _ => panic!("Invalid operator!"),
        };

        println!("Result is: {result}\n");
    }
}

fn flush() {
    std::io::stdout().flush().expect("Failed to flush output.");
}

fn parse_to_i32(s: &str) -> Result<i32, ParseIntError> {
    s.trim().parse()
}

struct Maths {
    digit1: i32,
    digit2: i32,
}

impl Maths {
    pub fn new(digit1: i32, digit2: i32) -> Maths {
        Maths { digit1, digit2 }
    }

    pub fn sum(&self) -> i32 {
        self.digit1 + self.digit2
    }

    pub fn subtract(&self) -> i32 {
        self.digit1 - self.digit2
    }

    pub fn multiply(&self) -> i32 {
        self.digit1 * self.digit2
    }

    pub fn divide(&self) -> i32 {
        self.digit1 / self.digit2
    }
}
