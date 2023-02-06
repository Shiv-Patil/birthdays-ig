mod commands;
mod structs;

fn main() {
    let mut chat_bot = structs::chatbot::ChatBot::new();
    chat_bot.register_command(structs::command::Command::new("add", "Add a birthday", "Takes 2 arguments (name and birthday). Adds the entry to the database.", commands::append::add_command));
    chat_bot.register_command(structs::command::Command::new("delete", "Remove a birthday", "Takes 1 required argument upto any number of optional arguments.", commands::delete::delete_command));
    chat_bot.register_command(structs::command::Command::new("list", "List the upcoming birthdays", "Lists the people and their birthdays whose birthdays are due today or tomorrow.\nUse with the argument `all` to display all birthdays.", commands::list::list_command));
    chat_bot.register_command(structs::command::Command::new("help", "Get info about a particular command", "Gets information about a particular command. Takes the command as an argument.", commands::help::help_command));
    match chat_bot.run() {
        Ok(()) => {},
        Err(e) => {println!("\nReadlineError: {}", e)}
    };
}
