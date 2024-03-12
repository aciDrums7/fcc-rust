// Make it work
pub fn main() {
    let f: bool = false;
    let t: bool = true && false;
    assert_eq!(f, t);

    println!("Success!");
}
