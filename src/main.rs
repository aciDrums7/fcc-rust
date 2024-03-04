//? If we don't explicitly assign a type to a variable, then the compiler will infer one for us.
#![allow(unused)]
fn main() {
    computations();
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

fn different_types_unreassignables() {
    let x: i32 = 5;
    // let mut y: u32 = 5;
    let mut y = 5;

    // We can't assign a var of a type another type
    y = x;

    let z = 10;

    println!("different_types_unreassignables()")
}

fn annotating_values() {
    let v: u16 = 38_u8 as u16;

    println!("annotating_values()")
}

fn types_checks() {
    let x: i32 = 5;
    // This will throw a validation error
    // assert_eq!("u32".to_string(), type_of(&x));

    assert_eq!("i32".to_string(), type_of(&x));

    println!("types_checks()");
}

fn max() {
    assert_eq!(u8::MAX, 255);
    assert_eq!(i8::MAX, 127);

    println!("Success!");
}

fn inferred() {
    // Overflow
    // let var1: u8 = 251_u8 + 8_u8;
    let var1: u16 = 251_u16 + 8;
    // This expression will panic
    // let var2 = i8::checked_add(251, 8).unwrap();
    let var2 = i16::checked_add(251, 2).unwrap();
    println!("{},{}", var1, var2);
}

fn numbers_representations() {
    // ? In order: decimal, hexadecimal, octal and binary
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111; // 1024 + 255 + 63 + 255
    assert!(v == 1597);

    print!("numbers_representations()");
}

fn floating_point() {
    let x = 1_000.000_1;
    let y: f32 = 0.12;
    let z = 0.01_f64;

    assert_eq!(type_of(&x), "f64".to_string());
    println!("floating_point()");
}

fn floating_point_precision() {
    // This assertion will fail because of the extreme precision of f64
    // assert!(0.1+0.2==0.3); // 0.1 + 0.2 ~= 0.3000000000002

    // Instead, this will work
    assert!((0.1 as f32) + (0.2 as f32) == (0.3 as f32));

    print!("floating_point");
}

fn for_cycle() {
    let mut sum: i32 = 0;

    for i in -3..=2 {
        sum += i;
    }

    assert!(sum == -3);

    // Printing the ASCII base 10 corrispondent to each char
    for c in 'a'..='z' {
        print!("{} ", c as u8);
    }
}

fn ranges() {
    // These are more verbose ways to express a range
    use std::ops::{ Range, RangeInclusive };
    assert_eq!(1..5, Range { start: 1, end: 5 });
    assert_eq!(1..=5, RangeInclusive::new(1, 5));

    println!("ranges()");
}

fn computations() {
    assert!(1u32 + 2 == 3);

    assert!(1i32 - 2 == -1);
    // assert!(1u8 - 2 == -1);
    assert!(1i8 - 2 == -1);

    assert!(3 * 50 == 150);

    assert!(9.6_f32 / 3.2_f32 == 3.0_f32);

    assert!(24 % 5 == 4);

    // Short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // Bitwise operations
    //^ each operation is made bit by bit (0 == false, 1 == true) -> bitwise! For instance:
    //^ 0 0 1 1 AND
    //^ 0 1 0 1
    //^ -------
    //^ 0 0 0 1
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    //^ 0 0 0 0 0 0 0 1 shifting 5 positions to left becomes
    //^ 0 0 1 0 0 0 0 0 == 32 in decimal
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}
