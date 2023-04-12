fn max_subarray_sum(arr: &[i32]) -> i32 {
    let mut max_sum = arr[0];
    let mut curr_sum = arr[0];
    
    for i in 1..arr.len() {
        curr_sum = curr_sum.max(0) + arr[i];
        max_sum = max_sum.max(curr_sum);
    }
    
    max_sum
}

fn main() {
    let arr = [1, -2, 3, 4, -5, 8];
    let max_sum = max_subarray_sum(&arr);
    println!("Maximum subarray sum: {}", max_sum);
}
