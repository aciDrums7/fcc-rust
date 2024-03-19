//^ Long tuples cannot be printed
pub fn main() {
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12); //? up to 12 elements to be printed
    println!("too long tuple: {:?}", too_long_tuple);
}
