use std::mem::size_of_val;
pub fn main() {
    let unit = ();

    println!("size of unit = {}", size_of_val(&unit));
    assert!(size_of_val(&unit) == 0);

    println!("unit_type_size()");
}
