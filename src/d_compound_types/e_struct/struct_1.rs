//^ We must specify concrete values for each of the fields in struct.
// Fix the error
struct Person {
    name: String,
    age: u8,
    hobby: String,
}

pub fn main() {
    let age: u8 = 30;
    // let p = Person {
    //     name: String::from("sunface"),
    //     age,
    // };
    let p: Person = Person {
        name: String::from("sunface"),
        age, //? this is a shorthand for age : age
        hobby: String::from("music"),
    };

    println!("Success!");
}
