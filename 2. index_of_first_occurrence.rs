fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() - 1;
    let mut result = None;
    
    while left <= right {
        let mid = (left + right) / 2;
        if arr[mid] == target {
            result = Some(mid);
            right = mid - 1;
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    
    result
}
fn main() {
    let arr = [1, 2, 3, 4, 5, 6];
    let target = 5;
    let index = find_first_occurrence(&arr, target);
    match index {
        Some(i) => println!("Found {} at index {}", target, i),
        None => println!("{} not found", target),
    }
}
