pub fn main() {
    let s: String = give_ownership();
    println!("{}", s);
}

// Only modify the code below!
fn give_ownership() -> String {
    let s: String = String::from("hello, world");

    // Convert String to Vec
    // ? into_bytes() converts any string into a bytes vector
    // ! and "consumes" the string, so `s` won't exist after calling it
    // let _s = s.into_bytes();

    let _s = s.as_bytes();
    s
}
