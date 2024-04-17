// Generate the nth Fibonacci number.
// The Fibonacci sequence is a series of numbers in which each number 
// is the sum of the two preceding ones, usually starting with 0 and 1.

use std::io;

fn fibonacci(n: u32) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut prev = 0;
    let mut curr = 1;

    for _ in 2..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }

    curr

}

fn main() {
    println!("Enter the value of n to find the nth Fibonacci number: ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let n: u32 = input.trim().parse().expect("Please enter a valid number");

    let result = fibonacci(n);
    println!("The {}th Fibonacci is: {}", n, result);
}
