#[derive(Debug)]
enum CarColour{
    Red,
    Green,
    Blue,
    Silver,
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
    }
}