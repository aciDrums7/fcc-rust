// Comment one line to make it work
pub fn main() {
    let mut s: String = String::from("hello, ");

    let r1: &mut String = &mut s;
    r1.push_str("world");

    // ? When declaring this other mut var referencing `s`, `r1` will be dropped out
    let r2: &mut String = &mut s;
    r2.push_str("!");
    //^ Not using `r1` after this line so we can have another reference (`r1` will be dropped)

    // println!("{}", r1);
    println!("{}", r2);

    println!("Success!")
}
