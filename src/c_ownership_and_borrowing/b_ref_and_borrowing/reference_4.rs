// Fix error
pub fn main() {
    let mut s: String = String::from("hello, ");

    // push_str(s);
    push_str(&mut s);

    println!("Success!");
}

fn push_str(s: &mut String) {
    s.push_str("world");
}
