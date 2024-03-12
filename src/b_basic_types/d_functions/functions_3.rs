// Solve it in two ways
// ! DON'T let `println!` work
pub fn main() {
    never_return();

    println!("Failed!");
}

// ! This is a diverging function: a function that NEVER returns back to the caller!
fn never_return() -> ! {
    // Implement this function, don't modify the fn sigrnature

    //^ panic macro cause the program to panic -> exiting returing an error
    panic!()
}
