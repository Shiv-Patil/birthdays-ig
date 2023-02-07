mod commands;
mod structs;
mod common;

fn main() {
    let mut chat_bot = structs::chatbot::ChatBot::new();
    chat_bot.register_command(commands::append::get_command());
    chat_bot.register_command(commands::delete::get_command());
    chat_bot.register_command(commands::list::get_command());
    chat_bot.register_command(commands::fetch::get_command());
    chat_bot.register_command(commands::help::get_command());
    match chat_bot.run() {
        Ok(()) => {},
        Err(e) => {println!("\nReadlineError: {}", e)}
    };
}
