//^ `String` type is defined in std and stored as vector of bytes (Vec), but guaranteed to always be a valid UTF-8 sequence.
//^ String is heap allocated, growable and not null terminated
// Fill the blank
pub fn main() {
    // let mut s = __;
    let mut s: String = String::from("");
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");

    println!("{}", s);

    println!("Success!");
}
