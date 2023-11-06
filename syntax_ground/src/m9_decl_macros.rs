// MACRO CAPTURES

/* expr
    matches to a valid rust expression
    "hello".to_string(), vec![1, 2, 3], 1 + 2, 1
*/

/* stmt
    matches to a rust statement
    let x = 5, x.push(1), return Some(x)
*/

/* ident
    matches to a rust identifier
    variable name, function name, module name
*/

/* ty
    matches to a rust type
    i32, Vec<String>, Option<T>
*/

/* path
    matches to a rust path
    std::collections::HashMap
*/

// REPITITION SPECIFIER

// * - match zero or more repititions
// + - match one or more repititions
// ? - Match zero or one repetition

#[cfg(test)]

mod tests{
    //  macro_rules! madskills {
    //     //  ($x: expr) => {
    //     //      format!("{} is mad skilled!", $x)
    //     //  };
    //     ($x: ty) => {
    //         match stringify!($X){
    //             "i32" => "i32 is mad skilled!".to_string(),
    //             _ => "I don't know this type".to_string()
    //         }
    //     };
    //  }

    macro_rules! my_vec {
        ($(x: expr),+ ) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )+
                temp_vec
            }
        };
    }


    #[test]
    fn tests_declarative_macro(){
        let mut _X: Vec<i32> = vec!();
        let mut y: Vec<i32> = my_vec!(1);

        println!("{:?}", x);
        // let some_var: String = madskills!(i32);
        // println!("{}", some_var);
    }
}