use actix_web::{web, App, HttpResponse, HttpServer};
use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
enum PrintCommand {
    AICall,
    UnitTest,
    Issue,
}

impl PrintCommand {
    pub fn print_agent_message(&self, agent_pos: &str, message: &str) -> String {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        let mut message_color = ColorSpec::new();
        message_color.set_fg(Some(Color::Green)).set_bold(true);
        let _ = stdout.set_color(&message_color);
        let _ = write!(stdout, "{}: ", agent_pos);
        message_color.set_fg(Some(Color::White)).set_bold(false);
        let _ = stdout.set_color(&message_color);
        let _ = writeln!(stdout, "{}", message);
        String::from(message)
    }

    pub fn get_user_response(&self, question: &str) -> String {
        let mut stdout: StandardStream = StandardStream::stdout(ColorChoice::Always);

        // Print the question in a specific color
        let mut question_color = ColorSpec::new();
        question_color.set_fg(Some(Color::Blue)).set_bold(true);
        let _ = stdout.set_color(&question_color);
        let _ = write!(stdout, "{}", question);

        // Reset Color
        let _ = stdout.set_color(&ColorSpec::new());

        // Read user input
        let mut human_response: String = String::new();
        let _ = io::stdin().read_line(&mut human_response);

        // Trim whitespace and convert to lowercase
        let human_response: String = human_response.trim().to_lowercase();

        human_response
    }

    pub fn confirm_safe_code(&self) -> String {
        let mut stdout: StandardStream = StandardStream::stdout(ColorChoice::Always);
        loop {
            // Print the question in specified color
            let mut question_color = ColorSpec::new();
            question_color.set_fg(Some(Color::Blue)).set_bold(true);
            let _ = stdout.set_color(&question_color);
            print!("WARNING: You are about to run code written entirely by AI. ");
            println!("Review your code and confirm you wish to continue.");

            // Reset Color
            let _ = stdout.set_color(&ColorSpec::new());

            // Present Options with different colors
            let mut option_color = ColorSpec::new();
            option_color.set_fg(Some(Color::Green)).set_bold(true);
            println!("[1] All good");
            option_color.set_fg(Some(Color::Red)).set_bold(true);
            println!("[2] Let's stop this project");

            // Reset Color
            let _ = stdout.set_color(&ColorSpec::new());

            // Read user input
            let mut human_response: String = String::new();
            let _ = io::stdin().read_line(&mut human_response);

            // Trim whitespace and convert to lowercase
            let human_response: String = human_response.trim().to_lowercase();

            // Match response
            let result = match human_response.as_str() {
                "1" | "ok" | "y" => "OK".to_string(),
                "2" | "no" | "n" => "Project stopped".to_string(),
                _ => "Invalid input. Please select '1' or '2'".to_string(),
            };

            return result;
        }
    }
}

async fn agent_message_handler(command: web::Path<PrintCommand>) -> HttpResponse {
    let agent_pos = "Web Agent"; // You can customize this as needed
    let agent_statement = match &command.into_inner() {
        PrintCommand::AICall => {
            let print_command = PrintCommand::AICall;
            print_command.print_agent_message(agent_pos, "AICall statement")
        }
        PrintCommand::UnitTest => {
            let print_command = PrintCommand::UnitTest;
            print_command.print_agent_message(agent_pos, "UnitTest statement")
        }
        PrintCommand::Issue => {
            let print_command = PrintCommand::Issue;
            print_command.print_agent_message(agent_pos, "Issue statement")
        }
    };

    HttpResponse::Ok().body(agent_statement)
}

async fn user_response_handler(command: web::Path<String>) -> HttpResponse {
    let question = "What is your response?"; // Customize the question as needed
    let user_response = PrintCommand::AICall.get_user_response(&question);
    HttpResponse::Ok().body(user_response)
}

async fn confirm_safe_code_handler(command: web::Path<String>) -> HttpResponse {
    let print_command = PrintCommand::AICall;
    let confirm_result = print_command.confirm_safe_code();
    HttpResponse::Ok().body(confirm_result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/agent/{command}").route(web::get().to(agent_message_handler)))
            .service(web::resource("/user/{command}").route(web::get().to(user_response_handler)))
            .service(web::resource("/confirm/{command}").route(web::get().to(confirm_safe_code_handler)))
    })
    .bind("localhost:8080")?
    .run()
    .await
}
