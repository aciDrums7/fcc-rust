//^ Opposite to the seldom using of `str`, `&str` and `String` are used everywhere!
//? `&str` can be converted to `String` in two ways
// Fix error with at least two solutions
pub fn main() {
    let s: &str = "hello, world";
    // greetings(s)
    // greetings(String::from(s))
    greetings(s.to_string()) //? &str -> String
}

fn greetings(s: String) {
    println!("{}", s)
}
