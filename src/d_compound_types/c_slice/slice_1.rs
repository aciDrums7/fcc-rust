//^ Here, both [i32] and str are slice types, but directly using it will cause errors.
//^ You have to use the reference of the slice instead: &[i32], &str.
// Fix the errors, DON'T add new lines!
pub fn main() {
    let arr: [i32; 3] = [1, 2, 3];
    // let s1: [i32] = arr[0..2];
    let s1: &[i32] = &arr[0..2]; // &[1, 2]

    // let s2: str = "hello, world" as str;
    let s2: &str = "hello, world" as &str;

    println!("Success!");
}
