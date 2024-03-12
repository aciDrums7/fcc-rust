pub fn main() {
    let x: i32 = 5;
    let y: &i32 = &x;

    // Modify this line only
    // assert_eq!(5, y);
    // ? The `*` is needed to point the value pointed by `y`, that is a reference var
    assert_eq!(5, *y);

    println!("Success!")
}
