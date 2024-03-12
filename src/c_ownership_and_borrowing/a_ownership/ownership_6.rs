//! Mutability can be changed when ownership is transferred
pub fn main() {
    let s: String = String::from("hello, ");

    // Modify this line only!
    // let s1 = s;
    let mut s1 = s;

    s1.push_str("world");

    println!("Success!")
}
