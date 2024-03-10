fn main() {
    print!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    }

    // Rather than returning a None, we use a diverging function instead
    never_return_fn();
}

//? Diverging functions never return to the caller, so they may be used in places where a value of any type is expected
fn never_return_fn() -> ! {
    //^ Could be implemented in various ways, here are 3 of 'em
    // todo!();
    // unimplemented!();
    panic!("I'll never return!");
}
