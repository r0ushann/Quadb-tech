use std::io;

fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }

    let limit = (n as f64).sqrt().floor() as i32;

    for i in 2..=limit {
        if n % i == 0 {
            return false;
        }
    }

    true
}

fn main() {
    println!("Enter a number:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let n = input.trim().parse::<i32>().expect("Invalid input");

    let result = if is_prime(n) {
        "is prime"
    } else {
        "is not prime"
    };

    println!("{} {}", n, result);
}
