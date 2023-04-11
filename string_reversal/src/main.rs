use std::io;

fn main() {
    println!("Enter a string to reverse:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let reversed = input.trim().chars().rev().collect::<String>();

    println!("Reversed string: {}", reversed);
}
