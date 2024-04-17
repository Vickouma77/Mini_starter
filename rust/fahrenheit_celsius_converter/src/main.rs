use std::io;

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

fn main() {
    println!("TEMPERATURE CONVERTER\n");
    println!("1. Fahrenheit to Celsius");
    println!("2. Celsius to Fahrenheit");
    println!("Choose an option (1 or 2): ");

    let mut options = String::new();

    io::stdin()
        .read_line(&mut options)
        .expect("Failed to read line");

    let options: u32 = options.trim().parse().expect("Please enter a number");

    match options {
        1 => {
            println!("Enter temperature in Fahrenheit: ");
            
            let mut fahrenheit_input = String::new();

            io::stdin()
                .read_line(&mut fahrenheit_input)
                .expect("Failed to read line");

            let fahrenheit: f64 = fahrenheit_input.trim().parse().expect("Enter a valid number");

            let celsius = fahrenheit_to_celsius(fahrenheit);
            println!("{}°F is {}°C", fahrenheit, celsius);
        }

        2 => {
            println!("Enter temperature in Celsius:");

            let mut celsius_input = String::new();

            io::stdin()
                .read_line(&mut celsius_input)
                .expect("Failed to read line");

            let celsius: f64 = celsius_input.trim().parse().expect("Please enter a valid number");

            let fahrenheit = celsius_to_fahrenheit(celsius);
            println!("{}°C is {}°F", celsius, fahrenheit);
        }

        _ => println!("Invalid option"),
    }
}
