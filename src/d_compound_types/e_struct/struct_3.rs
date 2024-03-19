//^ Tuple struct looks similar to tuples, it has added meaning the struct name provides but has no named fields.
//^ It's useful when you want to give the whole tuple a name, but don't care about the fields's names.
// Fix the error and fill the blanks
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
pub fn main() {
    // let v = Point(__, __, __);
    let v = Color(0, 127, 255);
    check_color(v);

    println!("Success!");
}

fn check_color(p: Color) {
    // let (x, _, _) = p;
    let Color(x, _, z) = p;
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    // assert_eq!(__, 255);
    assert_eq!(z, 255);
}
