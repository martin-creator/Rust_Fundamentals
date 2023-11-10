

pub fn extend_ai_function(ai_func: fn(&str) -> &'static str, func_input: &str) { //-> Message {
    let ai_function_str = ai_func(func_input);
}


#[cfg(test)]

mod tests {

    use super::*;
    use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;

    #[test]
    fn tests_extending_ai_function(){

        let x_str : &str = convert_user_input_to_goal("dummy variable")
    }
}