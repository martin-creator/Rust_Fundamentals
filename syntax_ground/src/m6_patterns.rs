







#[cfg(test)]

mod test {
    
    use super::*;

    #[test]
    fn test_patterns () {

        let number : i32 = 20;

        // match number {
        //     1 => print!("Ths is the first number!"),
        //     2|3|5|7|11|15|20 => print!("We found it in the sequence!"),
        //     13..=19 => print!("We found it in the range!"),
        //     _ => print!("We didn't find it!"),
        // }
       
       let res:&str = match number {
           1 => "Ths is the first number!",
           2|3|5|7|11|15|20 => "We found it in the sequence!",
           13..=19 => "We found it in the range!",
           _ => "We didn't find it!",      
         };

         print!("{}", res);
      }

        #[test]

        fn test_match_options(){

            let some_number: Option<i32> = None;
            //let prob_none: Option<i32> = None;

           let my_int:i32 = if let Some(i) = some_number {
            i
            } else {
                panic!("We found a None!")
            };

            print!("{}", my_int);

            // let res:i32 = match some_number{
            //     Some(i) => i,
            //     None => {
            //         panic!("We found a None!")
            //     }
            // };

            // print!("{}", res);
            // print!("{:?}", some_number);
            

        }

        #[test]

        fn tests_match_result() {

            let some_result: Result<i32, String> = Ok(42);
            let some_err: Result<i32, &str> = Err("We found an error!");

            let res = match some_res {
                Ok(i) => i,
                Err(e) => panic!("{}", e),
            };

            print!("{}", res);
        
            }
        }

