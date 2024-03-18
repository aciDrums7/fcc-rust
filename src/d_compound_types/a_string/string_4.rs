// Fix all errors without adding newline
pub fn main() {
    let mut s: String = String::from("hello");
    s.push(',');
    s.push_str(" world");
    s += "!";

    println!("{}", s);
}
