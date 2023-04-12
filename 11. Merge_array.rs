fn main() {
    let mut arr1 = [9, 7, 5, 3, 1];
    let mut arr2 = [10, 8, 6, 4, 2];
    let mut arr3 = [0;10];
    for i in 0..arr1.len() {
        arr3[i]=arr1[i];
    }
    let n = arr1.len();
    for j in 0..arr2.len() {
        arr3[n+j]=arr2[j];
    }
    println!("{:?}", arr3);
}
