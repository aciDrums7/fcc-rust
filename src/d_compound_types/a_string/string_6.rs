//^ More `String` methods can be found under String module
//? You can only concat a `String` with `&str`, and `String`'s ownership can be moved to another variable
pub fn main() {
    let s1: String = String::from("hello, ");
    let s2: String = String::from("world!");
    //  ! First argument must be of type `String`, while the other all of type `&str`
    // let s3 = s1 + s2;
    // let s3: String = s1 + s2.as_str(); // ? String -> &str
    let s3 = s1 + &s2;

    assert_eq!(s3, "hello, world!");

    // println!("{}", s1);
    println!("{}", s3);
}
