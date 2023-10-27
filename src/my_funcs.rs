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
