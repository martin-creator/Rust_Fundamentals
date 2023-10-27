/// Adds left and right and returns the result.
/// 
/// # Arguments
/// 
/// * `left` - The left hand side of the addition.
/// * `right` - The right hand side of the addition.
/// 
/// # Examples
/// 
/// ```
/// use martin_lib::add;
/// 
/// let result = add(2, 2);
/// assert_eq!(result, 4);
/// ```
/// 



pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
