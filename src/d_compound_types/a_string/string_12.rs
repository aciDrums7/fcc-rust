pub fn main() {
    // Fill the blank to print each char in "你好，世界"
    // for c in "你好，世界".__ {
    for c in "你好，世界".chars() {
        // ? `chars()` will put all the chars in the string literal into an iterator
        println!("{}", c);
    }
}
