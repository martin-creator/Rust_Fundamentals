use crate::models::general::llm::Message;


// Extend AI function to return certain specific messages
pub fn extend_ai_function(ai_func: fn(&str) -> &'static str, func_input: &str) -> Message {
    let ai_function_str = ai_func(func_input);
    
    // Extend the string to encourage only printing the output
    let msg: String = format!("FUNCTION: {}
    INSTRUCTION: You are a function printer. You only print the results of functions
    . Nothing else. No commentary. Here is the input to the function:{}.
    Print out what the function will return.", ai_function_str, func_input);

    // Return message
    Message {
        role: "system".to_string(),
        content: msg,
    }
}


#[cfg(test)]

mod tests {

    use super::*;
    use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;

    #[test]
    fn tests_extending_ai_function(){

       let extended_msg =  extend_ai_function(convert_user_input_to_goal, "dummy variable");
       assert_eq!(extended_msg.role, "system".to_string());
        
    }
}