fn main() {
    let mut arr = [5, 3, 8, 4, 2];
    let k = 2;
    arr.sort();
    println!("The number {} smallest position is: {}", k, arr[k-1]);
}
