//^ Members can be extracted from the tuple using indexing.
pub fn main() {
    let t: (&str, &str, &str) = ("i", "am", "sunface");
    // assert_eq!(t.1, "sunface");
    assert_eq!(t.2, "sunface");

    println!("Success!");
}
