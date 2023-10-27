pub fn divide_by_two(num: u32)->u32{
    num / 2
}


#[cfg(test)]

mod test {
    use super::*;

    #[test]

    fn divide_by_two_test(){
        let x:u32 = 100;
        let y = divide_by_two(x);
        print!("y is from divide_by_two_test: {} {}", y, x);
        assert_eq!(y, 50);
    }
}