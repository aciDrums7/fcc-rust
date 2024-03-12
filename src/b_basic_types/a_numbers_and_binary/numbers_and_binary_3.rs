// Modify `assert_eq!` to make it work
pub fn main() {
    let x = 5;
    // This will throw a validation error
    // assert_eq!("u32".to_string(), type_of(&x));

    assert_eq!("i32".to_string(), type_of(&x));

    println!("Success!");
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
