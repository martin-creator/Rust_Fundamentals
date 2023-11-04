#[derive(Debug)]
enum CarColour{
    Red,
    Green,
    Blue,
    Silver,
}


#[derive(Debug)]
enum  Given <T, E> {
    Success(T),
    Failure(E),
}

fn check_under_five(num_check: u8)-> Given<u8, String>{
    if num_check < 5 {
        Given::Success(num_check)
    } else {
        Given::Failure(String::from("Number is greater than 5"))
    }

}

fn create_car_colour_Red() -> CarColour {
    let my_car_colour = CarColour::Red; // this is a copy
    my_car_colour
}






#[cfg(test)]

mod test {

    use super::*;

    #[test]
    fn test_enums() {
        let my_car_colour = create_car_colour_Red();
        println!("my_car_colour = {:?}", my_car_colour);

        let under_five = check_under_five(4);
        dbg!(&under_five);
    }
}