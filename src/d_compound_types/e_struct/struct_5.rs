//^ Using field init shorthand syntax to reduce repetitions.
struct Person {
    name: String,
    age: u8,
}
pub fn main() {
    println!("Success!");
}

fn build_person(name: String, age: u8) -> Person {
    Person {
        age,
        // __,
        name, //? this is a shorthand for name: name
    }
}
