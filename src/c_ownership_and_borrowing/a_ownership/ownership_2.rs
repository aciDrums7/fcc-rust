// Don't modify code in main!
pub fn main() {
    let s1: String = String::from("hello, world");
    //^ s1 loses ownership
    let s2: String = take_ownership(s1);

    println!("{}", s2);
}

// Only modify the code below!
fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}
