fn main() {
    let (x, y);
    (x, ..) = (3, 4); // ".." means we don't care of the value at that index, so ignore it
    [.., y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x, y], [3, 2]);

    println!("Success!")
}
