#![allow(dead_code, unused_variables, unreachable_code, unused_assignments, unused_mut)]
pub mod a_variables;
pub mod b_basic_types;
pub mod c_ownership_and_borrowing;

//^ The main() fn is the starting point of every rust program
pub fn main() {
    c_ownership_and_borrowing::ref_and_borrowing::borrowing_8::main();
}
