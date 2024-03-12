fn main() {
    //? Box allows to allocate on heap, even raw values (usually allocated on stack)
    let x: Box<i32> = Box::new(5);

    // let ... // Implement this line, don't change other lines!
    let mut y: Box<i32> = Box::new(1);

    //? `*` is a reference to the value (stored in heap) pointed by the var `x`
    //^ so we're just reassigning a value here, but in heap insted of stack
    *y = 4;

    assert_eq!(*x, 5);

    println!("Success!")
}
