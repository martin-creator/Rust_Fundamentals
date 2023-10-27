mod my_funcs;

use crate::my_funcs::{add_five, divide_by_two};

// This is the main function
fn main() {
    let x:u32 = 10;
    let mut a = add_five(x);
    let b:u32 = divide_by_two(a);
    
    a = 100;

    println!("The value of a is: {}", a);
    println!("The value of b is: {}", b);
}
