fn main() {
    let array = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];

    let max_sum = max_subarray_sum(&array);

    println!("Maximum subarray sum: {}", max_sum);
}

fn max_subarray_sum(array: &[i32]) -> i32 {
    let mut max_so_far = i32::MIN;
    let mut max_ending_here = 0;

    for &num in array {
        max_ending_here += num;

        if max_ending_here > max_so_far {
            max_so_far = max_ending_here;
        }

        if max_ending_here < 0 {
            max_ending_here = 0;
        }
    }

    max_so_far
}
