use std::io::{self, Write};

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operation {
    fn from_str(op: &str) -> Option<Operation> {
        match op {
            "+" => Some(Operation::Add),
            "-" => Some(Operation::Subtract),
            "*" => Some(Operation::Multiply),
            "/" => Some(Operation::Divide),
            _ => None,
        }
    }

    fn calculate(&self, a: i32, b: i32) -> Option<i32> {
        match self {
            Operation::Add => Some(a + b),
            Operation::Subtract => Some(a - b),
            Operation::Multiply => Some(a * b),
            Operation::Divide => if b != 0 { Some(a / b) } else { None },
        }
    }
}

fn main() {
    let mut input = String::new();
    print!("Enter the operation and numbers as '5+6': ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();

    let operator_index = input
        .find(|c: char| !c.is_numeric())
        .unwrap_or_else(|| {
            println!("Invalid input.");
            std::process::exit(1);
        });

    let a: i32 = match input[..operator_index].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid first number.");
            return;
        }
    };

    let op = Operation::from_str(&input[operator_index..=operator_index]);
    if op.is_none() {
        println!("Invalid operation.");
        return;
    }

    let b: i32 = match input[operator_index + 1..].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid second number.");
            return;
        }
    };

    let result = op.unwrap().calculate(a, b);
    match result {
        Some(n) => println!("Result: {}", n),
        None => println!("Error in calculation, possibly due to division by zero."),
    }
}
