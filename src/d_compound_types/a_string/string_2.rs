//^ We cam only use `str` by boxed it, & can be used to convert `Box<str>` to `&str`
// Fix the error with at least two solutions
pub fn main() {
    // let s: Box<str> = "hello, world".into(); //? `into()` converts anything to the var annotated type
    // greetins(&s)
    let s: &str = "hello world";
    greetins(s)
}

fn greetins(s: &str) {
    println!("{}", s)
}
