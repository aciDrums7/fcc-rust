fn main() {
    let x: i32; // Uninitialized but used, ERROR !
    let _y: i32; // Uninitialized but also unused, only a Warning !
    let x: i32 = 5; // Now it works properly

    assert_eq!(x, 5);
    println!("Success!");
}
