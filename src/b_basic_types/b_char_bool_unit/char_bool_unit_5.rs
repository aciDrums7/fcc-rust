// Make it work, don't modify `implicitly_ret_unit` !
pub fn main() {
    //^ Unit type is an empty tuple of size 0 bytes; it's used to return "nothing" in expressions or functions
    let _v: () = ();

    let v = (2, 3); //^ Comparing with this var would falure
    assert_eq!(_v, implicitly_ret_unit());

    println!("Success!");
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}

// ! Don't use this one
fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}
