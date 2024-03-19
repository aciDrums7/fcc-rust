//^ Destructuring tuple with pattern
pub fn main() {
    let tup: (i32, f64, &str) = (1, 6.4, "hello");

    // Fill the blank to make the code work
    // let __ = tup;
    let (x, z, y) = tup;

    assert_eq!(x, 1);
    assert_eq!(y, "hello");
    assert_eq!(z, 6.4);

    println!("Success!");
}
