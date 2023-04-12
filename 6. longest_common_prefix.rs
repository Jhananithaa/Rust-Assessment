fn longest_common_prefix(strs: &[&str]) -> String {
    if strs.is_empty() {
        return String::new();
    }
    let mut prefix = String::from(strs[0]);
    for s in &strs[1..] {
        while !s.starts_with(&prefix) {
            prefix.pop();
            if prefix.is_empty() {
                return String::new();
            }
        }
    }
    prefix
}
fn main() {
    let strs = ["flower", "flow", "flight"];
    let prefix = longest_common_prefix(&strs);
    println!("The longest common prefix is '{}'", prefix);
}
