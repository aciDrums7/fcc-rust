//? `ref` can be used to take references to a value, similar to `&`
pub fn main() {
    let c: char = '中';

    let r1: &char = &c;
    // Fill the blank, dont change other code
    // let &r2: &char = &c;
    let ref r2: char = c;

    assert_eq!(*r1, *r2);

    // Check the equality of the two address strings
    assert_eq!(get_addr(r1), get_addr(r2));

    println!("Success!")
}

// Get memory address string
fn get_addr(r: &char) -> String {
    //? `format!` macro is similar to `println!` but instead
    //?  of printing to the defualt output it will return a String
    format!("{:p}", r)
}
