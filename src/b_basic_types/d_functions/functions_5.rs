fn main() {
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
