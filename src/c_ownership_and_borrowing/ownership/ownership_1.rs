//^ The program can access the stack memory at constant time O(1)
pub fn main() {
    // Use as many approaches as you can to make it work
    let x = String::from("hello, world");
    // let y: String = x;
    let y: String = x.clone();
    println!("{},{}", x, y);
}
