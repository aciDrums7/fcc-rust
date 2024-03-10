// Remove a line in the code to make it compile
fn shadowing_and_rebinding() {
    let mut x: i32 = 1;
    x = 7;
    // let x = x; //Shadowing and re-binding will throw an error here, because vars in Rust are immutable by nature

    // Shadowing and re-binding
    x += 3;
    println!("x = {}", x);

    let _y = 4;
    // Shadowing
    let _y = "I can also be bound to text!";

    println!("Success!");
}
