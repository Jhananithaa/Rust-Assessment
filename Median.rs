fn main() {
    let numbers = [1, 2, 3, 4, 5, 6];
    let n=numbers.len();
    let mid;
    if n==0 {
        println!("Given array is empty");
    }
    if n%2==0 {
        mid=n/2;
        println!("Median of the array is {}",numbers[mid]+numbers[mid-1]);
    }
    else {
        mid=n/2;
        println!("Median of the array is {}",numbers[mid]);
    }
}
