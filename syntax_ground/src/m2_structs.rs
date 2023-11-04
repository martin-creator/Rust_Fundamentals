#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn increment_sign_in_count(&mut self) {
        self.sign_in_count += 1;
    }

    fn change_email(&mut self, new_email: String) {
        self.email = String::from(new_email);
    }

    fn change_username(&mut self, new_username: String) {
        self.username = String::from(new_username);
    }
}


fn change_username(user: &mut User, new_username: String) {
    user.username = String::from(new_username);
}



#[cfg(test)]

mod test {
    use super::*;

    #[test]
    
    fn tests_structs(){
        let mut user_1 = User {
            email: String::from("martin@email.com"),
            username: String::from("martin"),
            active: true,
            sign_in_count: 1,
        };

        //user_1.email = String::from("moh@gmail.ocom");

        change_username(&mut user_1, String::from("moh"));

        dbg!(user_1);
        let mut user_2 = User {
            email: String::from("goh@gmail.com"),
            username: String::from("goh"),
            active: true,
            sign_in_count: 1,
        };

        user_2.increment_sign_in_count();
        user_2.change_email(String::from("hoh@gmail.com"));
        user_2.change_username(String::from("hoh"));

        dbg!(user_2);

       
        
    }
}