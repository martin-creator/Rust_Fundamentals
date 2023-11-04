trait Attacker{
    fn choose_style(&self) -> String;
}


#[derive(Debug)]
enum Character {
    Warrior,
    Mage,
    Rogue,
}

impl Attacker for Character {
    fn choose_style(&self) -> String {
        match self {
            Character::Warrior => String::from("wing chun"),
            Character::Mage => String::from("karate"),
            Character::Rogue => String::from("kung fu"),
        }
    }
}





#[cfg(test)]

mod test{
    use super::*;

    #[test]
    fn test_traits () {

        let warrior = Character::Warrior;
        let mage = Character::Mage;
        let rogue = Character::Rogue;

        println!("The warrior uses {}", warrior.choose_style());
        println!("The mage uses {}", mage.choose_style());
        println!("The rogue uses {}", rogue.choose_style());

    }
}