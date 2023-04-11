use std::io;

fn find_kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    if k > arr.len() {
        return None;
    }

    let mut arr = arr.to_vec();

    arr.sort();

    Some(arr[k - 1])
}

fn main() {
    println!("Enter an array of integers (separated by commas):");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let arr: Vec<i32> = input
        .split(",")
        .map(|s| s.trim().parse().expect("Invalid integer"))
        .collect();

    println!("Enter k:");

    let mut k_input = String::new();
    io::stdin().read_line(&mut k_input).expect("Failed to read line");

    let k: usize = k_input.trim().parse().expect("Invalid integer");

    match find_kth_smallest(&arr, k) {
        Some(kth_smallest) => println!("The kth smallest element is: {}", kth_smallest),
        None => println!("There is no kth smallest element in the array."),
    }
}
