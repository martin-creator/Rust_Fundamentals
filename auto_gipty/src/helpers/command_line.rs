use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
use std::io::{stdin, stdout, Write};



pub fn get_user_response(question: &str) -> String {
 
 // get user request
 let mut stdout: std::io::Stdout = std::io::stdout();

 // Print the question in a specific color
 stdout
     .execute(SetForegroundColor(Color::Blue));
     println!("");
     print!("{}", question);

    // Reset the color to the default color
    stdout.execute(ResetColor).unwrap();

    // Read user input
    let mut user_response: String = String::new();
    stdin()
        .read_line(&mut user_response)
        .expect("Did not enter a correct string");

    // Return the user response
    return user_response.trim().to_string();


}