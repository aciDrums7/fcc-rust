fn main() {
    let mut sum: i32 = 0;

    for i in -3..=2 {
        sum += i;
    }

    println!("sum = {}", sum);

    assert!(sum == -3);

    // Printing the ASCII base 10 corrispondent to each char
    for c in 'a'..='z' {
        print!("{} ", c as u8);
    }
}
