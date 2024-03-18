//^ `&String` can be implicitly converted into `&str`
// Fix errors
pub fn main() {
    let mut s: String = String::from("hello world");

    // Here, &s is `&String` type, but `first_letter` needs a `&str` type.
    // It works because `&String` can be implicitly converted to `&str. If you want to know more, this is called `Deref coercion`.
    let letter: &str = first_letter(&s); //? &String -> &str
    println!("the first letter is: {}", letter);

    s.clear(); // error!

    // println!("the first letter is: {}", letter); // ! Calling here letter will make the `s.clear` throw error
}

fn first_letter(s: &str) -> &str {
    &s[..1]
}
