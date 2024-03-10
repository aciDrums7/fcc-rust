fn main() {
    // Invalid syntax
    // let v = (let x = 3);
    let v = {
        let x = 3;
        x
    };

    assert!(v == 3);

    print!("Success!");
}
