use std::collections::{HashMap, HashSet};



#[cfg(test)]

mod test{
    use super::*;


    #[test]
    fn test_hashmap(){

        let person_1: &str = "alice";
        let person_2: &str = "bob";

        // &str -> Person
        // u8 -> &str
        // &str -> u32

        let mut results_hm: HashMap<&str, u8> = HashMap::new();
        results_hm.insert(person_1, 55);
        results_hm.insert(person_2, 65);

        let test_res = results_hm.get(person_1);
        print!("test_res: {:?}", test_res);

        if results_hm.contains_key("alice"){
            print!("Alice is here!")
        }
    }

    #[test]

    fn tests_hashset() {

        let mut names_hs: HashSet<&str> = HashSet::new();
        names_hs.insert("alice");
        names_hs.insert("bob");
        names_hs.insert("jane");

        if names_hs.contains("bob"){
            print!("Bob is here!")
        }
    }

}