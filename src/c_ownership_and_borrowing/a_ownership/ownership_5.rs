// Don't use clone, use copy instead
pub fn main() {
    // let x: (i32, i32, (), String) = (1, 2, (), "hello".to_string());
    // let y: (i32, i32, (), String) = x;
    //? here we're using a `string literal` -> a string hardcoded in the binary itself -> it's immutable and fixed size
    let x: (i32, i32, (), &str) = (1, 2, (), "hello");
    //^ Raw types (the ones saved in stack memory) gets implicitly copied
    let y: (i32, i32, (), &str) = x;
    println!("{:?}, {:?}", x, y);
}
