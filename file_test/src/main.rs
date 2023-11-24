use promptly::{prompt, Message};

fn main() {
    let word = "StartRight";
    let animation_duration = std::time::Duration::from_millis(100);

    loop {
        clear_console();

        // Slide animation
        for i in 0..word.len() {
            print!(
                "{}{}{}",
                color_for_index(i),
                &word[i..],
                color_reset()
            );

            std::thread::sleep(animation_duration);

            clear_console();
        }

        // Reset animation
        std::thread::sleep(animation_duration);
        clear_console();

        // Handle user input
        let input: String = prompt(Message::new("Press Enter to continue or type 'q' to quit: "));
        if input.trim().to_lowercase() == "q" {
            break;
        }
    }
}

fn color_for_index(index: usize) -> String {
    let colors = ["\u{001b}[31m", "\u{001b}[32m", "\u{001b}[34m", "\u{001b}[33m", "\u{001b}[35m"];
    colors[index % colors.len()].to_string()
}

fn color_reset() -> String {
    "\u{001b}[0m".to_string()
}

fn clear_console() {
    print!("\u{001b}[2J\u{001b}[1;1H");
    std::io::stdout().flush().unwrap();
}
