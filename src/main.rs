fn add_five(num: u32)->u32{
    num + 5
}

fn main() {
    let x:u32 = 10;
    let a = add_five(x);

    println!("The value of a is: {}", a);
}
