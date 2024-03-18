//^ `replace` can be used to replace substring
// Fill the blank
pub fn main() {
    let s: String = String::from("I like dogs");
    // Allocate new memory and store the modified string here
    // let s1 = s.__("dogs", "cats");
    let s1 = s.replace("dogs", "cats");

    assert_eq!(s1, "I like cats");

    println!("Success!");
}
