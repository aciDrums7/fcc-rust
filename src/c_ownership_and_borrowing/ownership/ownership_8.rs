fn main() {
    let t: (String, String) = (String::from("hello"), String::from("world"));

    //? Tuples are 0 index data structures, like arrays, so we can access any value at given index
    let _s = t.0;
    //^ After this instruction, t is not anymore the owner of the entire tuple, so the ending println! will throw error

    // Modify this line only, don't use `_s`
    // println!("{:?}", t);
    println!("{:?}", t.1);
}
