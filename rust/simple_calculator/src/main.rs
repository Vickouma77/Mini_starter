use std::io;

fn main() {
    println!("Simple Calculator");

    loop {
        println!("Enter operation (+, -, *, /) or 'q' to quit:");
        let mut operation = String::new();
        io::stdin().read_line(&mut operation).expect("Failed to read line");

        if operation.trim() == "q" {
            println!("Exiting...");
            break;
        }

        println!("Enter first number:");
        let mut num1 = String::new();
        io::stdin().read_line(&mut num1).expect("Failed to read line");
        let num1: f64 = num1.trim().parse().expect("Please enter a number");

        println!("Enter second number:");
        let mut num2 = String::new();
        io::stdin().read_line(&mut num2).expect("Failed to read line");
        let num2: f64 = num2.trim().parse().expect("Please enter a number");

        let result = match operation.trim() {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => {
                if num2 == 0.0 {
                    println!("Error: Division by zero");
                    continue;
                }
                num1 / num2
            }
            _ => {
                println!("Error: Invalid operation");
                continue;
            }
        };

        println!("Result: {}", result);
    }
}

