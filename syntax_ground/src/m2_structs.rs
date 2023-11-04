#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}




#[cfg(test)]

mod test {
    use super::*;

    #[test]
    
    fn tests_structs(){
        let user_1 = User {
            email: String::from("martin@email.com"),
            username: String::from("martin"),
            active: true,
            sign_in_count: 1,
        };

        dbg!(user_1);
        
    }
}