use std::io;

fn is_prime(number: u64) -> bool {
    if number <= 1 {
       return  false;
    }

    for i in 2..=(number as f64).sqrt() as u64 {
        if number % i == 0 {
           return  false;
        }
    }
    true
}

fn main() {
    println!("Enter a number: ");

    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Enter a valid number");

    let input: u64 = num.trim().parse().expect("Enter a valid number");

    if is_prime(input) {
        println!("{} is a prime number.", input);
    } else {
        println!("{} is not a prime number.", input);
    }
}

