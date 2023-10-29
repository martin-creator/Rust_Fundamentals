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

    // While loop
    let mut z = 0;
    while z < 5 {
        println!("z = {}", z);
        z += 1;
    }

    // If statement
    if z == 5 {
        println!("z is 5");
    } else {
        println!("z is not 5");
    }

    // if else if else
    if z == 5 {
        println!("z is 5");
    } else if z == 6 {
        println!("z is 6");
    } else {
        println!("z is not 5 or 6");
    }

    // match statement
    match z {
        1 => println!("z is 1"),
        2 => println!("z is 2"),
        3 => println!("z is 3"),
        4 => println!("z is 4"),
        5 => println!("z is 5"),
        6 => println!("z is 6"),
        _ => println!("z is not 1-6"),
    }

    // function call
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }
    let sum = add(2, 3);
    println!("sum = {}", sum);

    // ownership
    let s1 = String::from("Hello");
    let s2 = s1;
    // println!("{}", s1); // This will not work because s1 has been moved to s2
    println!("{}", s2);

    // borrowing
    let s3 = String::from("Hello");
    let s4 = &s3;
    println!("{}", s3);

    // mutable borrowing
    let mut s5 = String::from("Hello");
    let s6 = &mut s5;
    s6.push_str(", world!");
    println!("{}", s6);

    // structs
    struct Person {
        name: String,
        age: u8,
    }

    let p1 = Person {
        name: String::from("John"),
        age: 42,
    };

    println!("p1's name is {}", p1.name);

    // enums
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    let player_direction: Direction = Direction::Down;

    match player_direction {
        Direction::Up => println!("We are heading up!"),
        Direction::Down => println!("We are heading down!"),
        Direction::Left => println!("We are heading left!"),
        Direction::Right => println!("We are heading right!"),
    }

    // vectors
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    println!("v = {:?}", v);

    // closures
    let square = |x: i32| x * x;
    println!("square(2) = {}", square(2));

    // traits
    trait Animal {
        fn name(&self) -> &'static str;
        fn talk(&self) {
            println!("{} cannot talk", self.name());
        }
    }

    struct Human {
        name: &'static str,
    }

    impl Animal for Human {
        fn name(&self) -> &'static str {
            self.name
        }

        fn talk(&self) {
            println!("{} says hello", self.name());
        }
    }

    let h = Human { name: "John" };
    h.talk();

let freezing_temp = -2.4;
let boiling_temp = 100.0;

if freezing_temp < 0.0 {
    println!("Water is frozen")
} else if boiling_temp > 100.0 {
    println!("Water is boiling");
} else {
    println!("Water is liquid");
}

let my_inits: [char; 3] = ['J', 'C', 'D']; // This is a static array stored on the stack
println!("My initials are {}{}{}", my_inits[0], my_inits[1], my_inits[2]);

let my_floats = [1.0, 2.0, 3.0]; // This is a static array stored on the stack
let my_new_floats = my_floats.map(|x: f64| x * 2.0); // This is a static array stored on the stack
println!("my_new_floats = {:?}", my_new_floats);

}
