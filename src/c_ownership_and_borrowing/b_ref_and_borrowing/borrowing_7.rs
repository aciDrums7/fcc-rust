// Remove something to make it work
// Don't remove a whole line!
pub fn main() {
    let mut s: String = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s;
    let r1: &String = &s;
    let ref r2: String = s;

    println!("{}, {}", r1, r2);

    // ! `s` is still the owner of the value
    println!("{}", s);

    println!("Success!")
}
