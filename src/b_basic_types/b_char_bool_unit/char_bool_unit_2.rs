// Make it work
fn main() {
    // let c1: char = "中"; //^ double quotes for string
    let c1: char = '中'; //^ single quotes for chars
    print_char(c1);

    fn print_char(c: char) {
        println!("{}", c);
    }
}
