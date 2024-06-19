//palindrome checker

fn is_palindrome(s: &str) -> bool {
    let s = s.to_lowercase();
    let s = s.chars().filter(|c| c.is_alphanumeric()).collect::<String>();
    let reversed = s.chars().rev().collect::<String>();
    s == reversed
}

fn main() {
    let input = "A man, a plan, a canal, Panama!";
    println!("Is \"{}\" a palindrome? {}", input, is_palindrome(input));
}

