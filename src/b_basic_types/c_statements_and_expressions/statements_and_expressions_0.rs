pub fn main() {
    let x: u32 = 5u32;

    //^ This is a statement, since ends with semicolon and doesn't produce a value
    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression since its evalutation will be assigned to 'y'
        // ! Notice it hasn't the final semicolon
        x_cube + x_squared + x
    };

    let z = {
        // The semicolon suppresses this expression and '()' is assigned to 'z'
        // 2 * x;
        // ^ This way it becomes an expression
        2 * x
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
