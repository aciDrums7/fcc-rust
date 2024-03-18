// This code has no errors!
pub fn main() {
    let mut s: String = String::from("hello, ");

    borrow_object(&s);

    s.push_str("world");

    println!("Success!")
}

fn borrow_object(s: &String) {}
