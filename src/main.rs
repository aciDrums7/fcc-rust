#![allow(unused)]
fn main() {
    match_like_switch();
}

fn functions() {
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);

    println!("functions()");

    // We must explicit type for functions params
    fn sum(x: i32, y: i32) -> i32 {
        x + y
    }
}

fn print_statement() -> () {
    println!("print_statement()");
}

fn never_return() {
    never_return();

    println!("never_return() failed");

    // ! This is a diverging function: a function that NEVER returns back to the caller!
    fn never_return() -> ! {
        //^ panic macro cause the program to panic -> exiting returing an error
        panic!()
    }
}

//? Diverging functions never return to the caller, so they may be used in places where a value of any type is expected
fn diverging_functions() {
    /* fn get_option(tp: u8) -> Option<i32> {
        match tp {
            1 => {
                // TODO
            }
            _ => {
                // TODO
            }
        }
    } */
    never_return_fn();

    // ? Could be implemented in various ways, here are 3 of 'em
    fn never_return_fn() -> ! {
        // panic!();
        // unimplemented!();
        todo!();
    }
}

fn match_like_switch() {
    let b = false;

    let _v = match b {
        true => 1,
        // Diverging functions can also be used in match expressions to replace a value of any value
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic");
        }
    };

    println!("Exercise failed if print out this line!");
}
