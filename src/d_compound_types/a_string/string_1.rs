//^ We can't use `str` type in normal ways, but we can use `&str`
// Fix error without adding new line
pub fn main() {
    // let s: str = "hello, world";
    let s: &str = "hello, world";

    println!("Success!");
}
