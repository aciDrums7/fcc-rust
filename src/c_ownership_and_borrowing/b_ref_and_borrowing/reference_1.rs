pub fn main() {
    let x: i32 = 5;
    // FIll the blank
    // let p = __;
    let p: &i32 = &x;

    //? `{:p}` is used when printing a reference var
    println!("The memory address of x is {:p}", p);
}
