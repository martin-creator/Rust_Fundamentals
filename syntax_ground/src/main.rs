const OUR_COURSE: &str = "Rust Programming Language"; // This string is a static string stored on the stack

fn main() {
    println!("Welcome to this course on {}", OUR_COURSE);

    // stack
    let x: i32;
    x = 42;
    println!("x = {}", x);

    // heap
    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{}", s);


    // stack
    let y: i32;
    y = 4;
    println!("y = {}", y);

    // For loop
    for i in 0..=y {
        if i % 2 == 0 {
            println!("{} is even", i);
        } else {
            println!("{} is odd", i);
        }
    }
}
