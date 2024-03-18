pub fn main() {
    let mut s: String = String::from("hello, ");

    // borrow_objects(s);
    borrow_objects(&s);

    println!("Success!");
}

fn borrow_objects(s: &String) {}
