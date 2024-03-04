#![allow(unused)]
fn main() {
    sum();
}

fn statements_and_expressions() {
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

fn ex_1() {
    let v = {
        let mut x = 1;
        x += 2;
        // This way we'll have an evaluation
        x
    };

    assert_eq!(v, 3);

    println!("ex_1()");
}

fn ex_2() {
    // Invalid syntax
    // let v = (let x = 3);
    let v = {
        let x = 3;
        x
    };

    assert!(v == 3);

    print!("ex_2()");
}

fn sum() {
    let s = sum(1, 2);
    assert_eq!(s, 3);

    fn sum(x: i32, y: i32) -> i32 {
        // This way, nothing gets returned
        // x + y;
        x + y
    }

    println!("sum()");
}
