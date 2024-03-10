// Fill the blanks and fix the errors
fn main() {
    // Integer addition
    assert!(1u32 + 2 == 3);

    // Integer subtraction
    assert!(1i32 - 2 == -1);
    // assert!(1u8 - 2 == -1);
    assert!(1i8 - 2 == -1);

    assert!(3 * 50 == 150);

    // assert!(9.6f32 / 3.2f32 == 3.0f32); // error ! make it work
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
