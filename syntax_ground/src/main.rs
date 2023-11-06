mod m1_enums;
mod m2_structs;
mod m3_traits;
mod m4_polymorphism;
mod m5_lifetimes;
mod m6_patterns;
mod m7_async;
mod m8_collections;
mod m9_decl_macros;
use std::vec;

const OUR_COURSE: &str = "Rust Programming Language"; // This string is a static string stored on the stack

fn main() {
//     println!("Welcome to this course on {}", OUR_COURSE);

//     // stack
//     let x: i32;
//     x = 42;
//     println!("x = {}", x);

//     // heap
//     let mut s = String::from("Hello");
//     s.push_str(", world!");
//     println!("{}", s);


//     // stack
//     let y: i32;
//     y = 4;
//     println!("y = {}", y);

//     // For loop
//     for i in 0..=y {
//         if i % 2 == 0 {
//             println!("{} is even", i);
//         } else {
//             println!("{} is odd", i);
//         }
//     }

//     // While loop
//     let mut z = 0;
//     while z < 5 {
//         println!("z = {}", z);
//         z += 1;
//     }

//     // If statement
//     if z == 5 {
//         println!("z is 5");
//     } else {
//         println!("z is not 5");
//     }

//     // if else if else
//     if z == 5 {
//         println!("z is 5");
//     } else if z == 6 {
//         println!("z is 6");
//     } else {
//         println!("z is not 5 or 6");
//     }

//     // match statement
//     match z {
//         1 => println!("z is 1"),
//         2 => println!("z is 2"),
//         3 => println!("z is 3"),
//         4 => println!("z is 4"),
//         5 => println!("z is 5"),
//         6 => println!("z is 6"),
//         _ => println!("z is not 1-6"),
//     }

//     // function call
//     fn add(x: i32, y: i32) -> i32 {
//         x + y
//     }
//     let sum = add(2, 3);
//     println!("sum = {}", sum);

//     // ownership
//     let s1 = String::from("Hello");
//     let s2 = s1;
//     // println!("{}", s1); // This will not work because s1 has been moved to s2
//     println!("{}", s2);

//     // borrowing
//     let s3 = String::from("Hello");
//     let s4 = &s3;
//     println!("{}", s3);

//     // mutable borrowing
//     let mut s5 = String::from("Hello");
//     let s6 = &mut s5;
//     s6.push_str(", world!");
//     println!("{}", s6);

//     // structs
//     struct Person {
//         name: String,
//         age: u8,
//     }

//     let p1 = Person {
//         name: String::from("John"),
//         age: 42,
//     };

//     println!("p1's name is {}", p1.name);

//     // enums
//     enum Direction {
//         Up,
//         Down,
//         Left,
//         Right,
//     }

//     let player_direction: Direction = Direction::Down;

//     match player_direction {
//         Direction::Up => println!("We are heading up!"),
//         Direction::Down => println!("We are heading down!"),
//         Direction::Left => println!("We are heading left!"),
//         Direction::Right => println!("We are heading right!"),
//     }

//     // vectors
//     let mut v: Vec<i32> = Vec::new();
//     v.push(5);
//     v.push(6);
//     v.push(7);
//     println!("v = {:?}", v);

//     // closures
//     let square = |x: i32| x * x;
//     println!("square(2) = {}", square(2));

//     // traits
//     trait Animal {
//         fn name(&self) -> &'static str;
//         fn talk(&self) {
//             println!("{} cannot talk", self.name());
//         }
//     }

//     struct Human {
//         name: &'static str,
//     }

//     impl Animal for Human {
//         fn name(&self) -> &'static str {
//             self.name
//         }

//         fn talk(&self) {
//             println!("{} says hello", self.name());
//         }
//     }

//     let h = Human { name: "John" };
//     h.talk();

// let freezing_temp = -2.4;
// let boiling_temp = 100.0;

// if freezing_temp < 0.0 {
//     println!("Water is frozen")
// } else if boiling_temp > 100.0 {
//     println!("Water is boiling");
// } else {
//     println!("Water is liquid");
// }

// let my_inits: [char; 3] = ['J', 'C', 'D']; // This is a static array stored on the stack
// println!("My initials are {}{}{}", my_inits[0], my_inits[1], my_inits[2]);

// let my_floats = [1.0, 2.0, 3.0]; // This is a static array stored on the stack
// let my_new_floats = my_floats.map(|x: f64| x * 2.0); // This is a static array stored on the stack
// println!("my_new_floats = {:?}", my_new_floats);

// let name:&str = "Martin Lubowa";
// let age:u8 = 23;
// let is_tall:bool = true;
// println!("My name is {} and I am {} years old. Am I tall? {}", name, age, is_tall);

// let dynamic_name = String::from("Martin Lubowa");
// println!("My dynamic_name is {}", dynamic_name);
// print!("My dynamic_name is stored at {:p}", &dynamic_name);

// let str_slice:&str = &dynamic_name[0..6];
// println!("My str_slice is {}", str_slice);


// let mut chars: Vec<char> = Vec::new();
// chars.insert(0, 'M');
// chars.insert(1, 'a');
// chars.insert(2, 'r');
// chars.insert(3, 't');
// chars.insert(4, 'i');
// chars.insert(5, 'n');
// chars.push(' ');

// println!("{:?}", chars);
// dbg!(&chars);

// let removed_char = chars.pop().unwrap();
// println!("Removed char is {}", removed_char);
// println!("{:?}", chars);

// chars.iter().for_each(|c:&char| println!("char is {}", c));

// let chars_again:Vec<char> = vec!('J', 'O', 'S', 'H', 'U', 'A');
// dbg!(&chars_again);

// let collected: String = chars_again.iter().collect();
// println!("collected is {}", collected);

// for c in chars_again{
//     println!("char is {}", c);
//     if c == 'O'{
//         println!("Found O");
//     }
// }

// Closures
// let num: i32 = 5;
// let add_num = |x: i32| x + num;
// let new_num = add_num(5);
// println!("new_num is {}", new_num);

// // Number Literals  from Rust Book
// println!("BigNumber: {}", 1_000_000);
// println!("Hex: {}", 0xff);
// println!("Octal: {}", 0o77);
// println!("Binary: {}", 0b1111_0000);
// println!("Byte: {}", b'A');

// // Raw String Literals
// let text:&str = r#"{ "name": "Martin Lubowa", "age": 23 }"#;
// println!("text is {}", text);

// // Tuple
let tup: (i32, f64, char) = (500, 6.4, 'a');
let (x, y, z) = tup;
println!("The value of y is: {}", y);

// Binary
let bin = 0b1111_0000;
println!("bin is {}", bin);
let b: u8 = 0b1111_0000;
println!("b is {}", b);

// Logic Gates
println!("b in binary is {:08b}", b);
println!("b in binary is {:08b}", 12);

// new binary
let a:u8 = 0b1111_0000;
let b:u8 = 0b0000_1111;

// Logic gates with operators
println!("a & b = {:08b}", a & b);
println!("a | b = {:08b}", a | b);
println!("a ^ b = {:08b}", a ^ b);
println!("!a = {:08b}", !a);
println!("!b = {:08b}", !b);

// Bitwise shift operators
print!("a << 1 = {:08b}", a << 1);
println!("a >> 1 = {:08b}", a >> 1);
print!("b << 1 = {:08b}", b << 1);
println!("b >> 1 = {:08b}", b >> 1);
print!("~a << 1 = {:08b}", !a << 1);
println!("~a >> 1 = {:08b}", !a >> 1);

// Little Endian or Big Endian
let num:u16 = 0b0000_0000_0000_0001;
println!("num is {}", num);
println!("num in binary is {:016b}", num);

let big_endian: [u8; 2] = num.to_be_bytes();
println!("big_endian is {:?}", big_endian);




}
