fn find_shortest_word(s: &str) -> Option<&str> {
    s.split_whitespace().min_by_key(|word| word.len())
}
fn main() {
    let s = "Hello everyone! welcome to Rust programming";
    match find_shortest_word(s) {
        Some(word) => println!("Shortest word is: {}", word),
        None => println!("Empty input string"),
    }
}
