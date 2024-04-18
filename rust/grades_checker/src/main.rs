use std::io;

fn main() {
    println!("Enter scores: ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let score: u32 = input.trim().parse().expect("Please enter a valid number");

    let grade = match score {
        0..=59 => "F",
        60..=69 => "D",
        70..=79 => "C",
        80..=89 => "B",
        90..=100 => "A",
        _ => "Invalid score",
    };

    println!("The student's grade is: {}", grade);
}
