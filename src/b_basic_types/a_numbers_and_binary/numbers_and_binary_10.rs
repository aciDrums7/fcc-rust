// Fill the blanks
use std::ops::{ Range, RangeInclusive };
fn main() {
    // These are more verbose ways to express a range
    assert_eq!(1..5, Range { start: 1, end: 5 });
    assert_eq!(1..=5, RangeInclusive::new(1, 5));

    println!("Success!");
}
