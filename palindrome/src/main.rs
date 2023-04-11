use std::io;

fn is_palindrome(s: &str) -> bool {
    let s = s.to_lowercase();
    let s = s.chars().filter(|c| c.is_ascii_alphanumeric()).collect::<String>();
    let n = s.len();
    for i in 0..n/2 {
        if s.chars().nth(i).unwrap() != s.chars().nth(n-i-1).unwrap() {
            return false;
        }
    }
    true
}

fn main() {
    println!("Enter a string:");

    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");

    if is_palindrome(&s) {
        println!("{} is a palindrome", s.trim());
    } else {
        println!("{} is not a palindrome", s.trim());
    }
}
