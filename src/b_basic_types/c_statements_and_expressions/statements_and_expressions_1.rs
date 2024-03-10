// Make it work with two ways
fn main() {
    let v = {
        let mut x = 1;
        x += 2;
        // This way we'll have an evaluation
        x
    };

    assert_eq!(v, 3);

    println!("Success!");
}
