//* The main() fn is the starting point of every rust program
fn main() {
    destructuring_assignments();
}

fn uninitialized_error() {
    let x: i32; // Uninitialized but used, ERROR !
    let y: i32; // Uninitialized but also unused, only a Warning !
    let x: i32 = 5; // Now it works properly

    assert_eq!(x, 5);
    println!("uninitialized_error() is working");
}

fn mutable() {
    //! In Rust, by default, a variable is immutable! You have to declare it mutable using the keyword "mut"
    let mut x = 1;
    x += 2;

    assert_eq!(x, 3);
    println!("mutable() is working");
}

fn scope() {
    let x: i32 = 10;
    {
        //^ Since y is defined in an inner scope, the println! outside of this scope will throw error
        let y: i32 = 5; // not working outside of this scope
        println!("The value of x is {} and value of y is {}", x, y);
    }
    let y: i32 = 5; // now is working
    println!("The value of x is {} and value of y is {}", x, y);
}

fn define_x() {
    let x: &str = "hello";
    println!("{}, world", x);
}

fn shadowing() {
    // The first variable is "shadowed" by the second one
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x); // Prints "42"
}

fn shadowing_and_rebinding() {
    let mut x: i32 = 1;
    x = 7;
    // let x = x; //Shadowing and re-binding will throw an error here, because vars in Rust are immutable by nature
    x += 3;

    let y = 4;
    // Shadowing
    let y = "I can also be bound to text!";

    println!("shadowing_and_rebinding() working");
}

#[allow(unused_variables)]
fn remove_warnings() {
    let _x = 1; // Prepending with _ or using the "allow[]" will remove unused_variables warninng
}

fn tuples_destructuring() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("tuples_destructuring() is working");
}

fn destructuring_assignments() {
    let (x, y);
    (x, ..) = (3, 4); // ".." means we don't care of the value at that index, so ignore it
    [.., y] = [1, 2];
    assert_eq!([x, y], [3, 2]);

    println!("destructuring_assignments() working")
}
