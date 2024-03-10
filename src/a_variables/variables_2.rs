// Fill the blanks in the code to make it compile
fn mutable() {
    //! In Rust, by default, a variable is immutable! You have to declare it mutable using the keyword "mut"
    let mut x = 1;
    x += 2;

    assert_eq!(x, 3);
    println!("Success!");
}
