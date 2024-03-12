// Only modify the `assert_eq` to make the `println!` work (print `42` in terminal)
pub fn main() {
    // The first variable is "shadowed" by the second one
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x); // Prints "42"
}
