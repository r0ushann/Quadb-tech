fn main() {
    let array1 = vec![1, 45,56, 98];
    let array2 = vec![2, 43, 64, 85, 10];

    let merged_array = merge_arrays(&array1, &array2);

    println!("{:?}", merged_array);
}

fn merge_arrays(array1: &[i32], array2: &[i32]) -> Vec<i32> {
    let mut merged_array = vec![];
    let mut i = 0;
    let mut j = 0;

    while i < array1.len() && j < array2.len() {
        if array1[i] < array2[j] {
            merged_array.push(array1[i]);
            i += 1;
        } else {
            merged_array.push(array2[j]);
            j += 1;
        }
    }

    while i < array1.len() {
        merged_array.push(array1[i]);
        i += 1;
    }

    while j < array2.len() {
        merged_array.push(array2[j]);
        j += 1;
    }

    merged_array
}
