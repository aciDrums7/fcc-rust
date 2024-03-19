// Fix errors to make it work
#[derive(Debug)]
struct File {
    name: String,
    data: String,
}
pub fn main() {
    let f: File = File {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string(),
    };

    // let _name: String = f.name;
    let _name: String = f.name.clone();

    // ONLY modify this line
    // println!("{}, {}, {:?}", f.name, f.data, f);
    println!("{}, {}, {:?}", f.name, f.data, f);
}
