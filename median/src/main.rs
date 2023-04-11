use std::io;

fn find_median(arr: &[i32]) -> Option<f64> {
    let n = arr.len();

    if n == 0 {
        return None;
    }

    let mid = n / 2;

    if n % 2 == 0 {
        Some((arr[mid - 1] + arr[mid]) as f64 / 2.0)
    } else {
        Some(arr[mid] as f64)
    }
}

fn main() {
    println!("Enter a sorted array of integers (separated by commas):");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let arr: Vec<i32> = input
        .split(",")
        .map(|s| s.trim().parse().expect("Invalid input"))
        .collect();

    let median = find_median(&arr);

    match median {
        Some(value) => println!("The median is: {}", value),
        None => println!("The array is empty"),
    }
}
