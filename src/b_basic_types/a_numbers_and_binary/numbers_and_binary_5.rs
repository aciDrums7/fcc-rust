// Fix errors and panics to make it work
pub fn main() {
    // Overflow
    // let var1: u8 = 251_u8 + 8_u8;
    let var1: u16 = 251_u16 + 8;
    // This expression will panic
    // let var2 = i8::checked_add(251, 8).unwrap();
    let var2 = i16::checked_add(251, 2).unwrap();
    println!("{},{}", var1, var2);
}
