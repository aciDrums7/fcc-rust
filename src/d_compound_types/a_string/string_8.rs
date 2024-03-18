//^ We can use `String::from` or `to_string` to convert a `&str` to `String`
// Use two approaches to fix the error and without adding a new line
pub fn main() {
    let s: String = "hello, world".to_string();
    // let s1: &str = s;
    // let s1: &str = s.as_str();
    let s1: &str = &s; // ? &String -> &str

    println!("Success!");
}
