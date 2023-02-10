#![warn(unused_results)]
mod commands;
mod common;
mod structs;

fn main() {
    let mut chat_bot = structs::chatbot::ChatBot::new();
    chat_bot.register_command(commands::append::get_command());
    chat_bot.register_command(commands::import::get_command());
    chat_bot.register_command(commands::delete::get_command());
    chat_bot.register_command(commands::list::get_command());
    chat_bot.register_command(commands::fetch::get_command());
    chat_bot.register_command(commands::help::get_command());
    chat_bot.register_command(commands::quiz::get_command());
    chat_bot.register_command(commands::export::get_command());
    chat_bot.register_command(commands::wish::get_command());
    match chat_bot.run() {
        Ok(()) => {}
        Err(e) => {
            println!("\nReadlineError: {e}")
        }
    };
}
