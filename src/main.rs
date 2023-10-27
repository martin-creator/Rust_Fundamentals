fn add_five(num: u32)->u32{
    num + 5
}

fn divide_by_two(num: u32)->u32{
    num / 2
}

fn main() {
    let x:u32 = 10;
    let mut a = add_five(x);
    let mut b:u32 = divide_by_two(a);
    
    a = 100;

    println!("The value of a is: {}", a);
    println!("The value of b is: {}", b);
}
