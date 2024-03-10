fn main() {
    // This assertion will fail because of the extreme precision of f64
    // assert!(0.1+0.2==0.3); // 0.1 + 0.2 ~= 0.3000000000002

    // Instead, this will work
    assert!((0.1 as f32) + (0.2 as f32) == (0.3 as f32));

    print!("Success!");
}
