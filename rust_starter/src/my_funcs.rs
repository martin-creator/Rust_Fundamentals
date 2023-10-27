/// Function: add_five
/// 
/// This function adds 5 to the input number
/// 
/// # Arguments
/// 
/// * `num` - A 32 bit unsigned integer
/// 
/// # Example
/// 
/// ```
/// let x:u32 = 10;
/// let y = add_five(x);
/// ```
/// 
/// # Returns
/// 
/// * A 32 bit unsigned integer
/// 

/**
 * Function: add_five
 * This function adds 5 to the input number
 */
pub fn add_five(num: u32)->u32{
    num + 5
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]

    fn add_five_test(){
        let x:u32 = 100;
        let y = add_five(x);
        print!("y is from add_five_test: {} {}", y, x);
        assert_eq!(y, 105);
    }
}
