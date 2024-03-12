// Make it work
use std::mem::size_of_val; //^ returns size in bytes
pub fn main() {
    let c1: char = 'a';
    println!("{}", size_of_val(&c1));
    assert_eq!(size_of_val(&c1), 4);

    let c2: char = '中';
    println!("{}", size_of_val(&c2));
    assert_eq!(size_of_val(&c1), 4);

    println!("Success!");
}
