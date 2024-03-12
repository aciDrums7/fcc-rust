// Remove something to make it work
//? If we don't explicitly assign a type to a variable, then the compiler will infer one for us.
pub fn main() {
    let x: i32 = 5;
    // let mut y: u32 = 5;
    let mut y = 5;

    // We can't assign a var of a type another type
    y = x;

    let z = 10;

    println!("Success!")
}
