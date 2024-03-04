#![allow(unused)]
fn main() {
    unit_type_size();
}

fn char() {
    use std::mem::size_of_val; //^ returns size in bytes

    let c1: char = 'a';
    println!("{}", size_of_val(&c1));
    assert_eq!(size_of_val(&c1), 4);

    let c2: char = '中';
    println!("{}", size_of_val(&c2));
    assert_eq!(size_of_val(&c1), 4);

    println!("char()");
}

fn single_and_double_quotes() {
    // let c1: char = "中"; //^ double quotes for string
    let c1: char = '中'; //^ single quotes for chars
    print_char(c1);

    fn print_char(c: char) {
        println!("{}", c);
    }
}

fn bools() {
    let _f: bool = false;

    let t: bool = false;

    if !t {
        println!("bools()");
    }
}

fn bools_2() {
    let f: bool = false;
    let t: bool = true && false;
    assert_eq!(f, t);
    println!("bools_2()");
}

fn unit_type() {
    //^ Unit type is an empty tuple of size 0 bytes; it's used to return "nothing" in expressions or functions
    let _v: () = ();

    let v = (2, 3); //^ Comparing with this var would falure
    assert_eq!(_v, implicitly_ret_unit());

    fn implicitly_ret_unit() {
        println!("I will return ()");
    }
}

fn unit_type_size() {
    use std::mem::size_of_val;

    let unit = ();
    assert!(size_of_val(&unit) == 0);

    println!("unit_type_size()");
}
