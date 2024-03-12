// Fix the error without removing code line
fn main() {
    let s: String = String::from("hello, world");

    // print_str(s);
    print_str(s.clone());

    println!("{}", s);
}

// print_str take ownership of `s`
fn print_str(s: String) {
    println!("{}", s)
}
