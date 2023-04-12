fn main() {
    let x=21;
    if x<2 {
        println!("{} is not a prime number",x);
    }
    let mut c=1;
    for i in 2..x {
        if x%i==0 {
        c=0;
        break;
        }
    }
    if c==0 {
        println!("{} is not a prime number",x)
    }
    else {
        println!("{} is a prime number",x)
    }
}
