pub fn main() {
    // Fix error by modifying this line
    let mut s: String = String::from("hello, ");

    // borrow_objects(&mut s);
    borrow_objects(s);

    println!("Success!")
}

// fn borrow_objects(s: &mut String) {}
fn borrow_objects(ref mut s: String) {}
