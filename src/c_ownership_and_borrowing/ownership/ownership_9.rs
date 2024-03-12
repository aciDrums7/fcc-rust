fn main() {
    let t: (String, String) = (String::from("hello"), String::from("world"));

    // Fill th blanks
    // let (__, __) = __;
    // let (s1, s2) = t;
    let (s1, s2) = t.clone();
    //^ `t` doesn't own any value after this instruction

    println!("{:?}, {:?}, {:?}", s1, s2, t)
}
