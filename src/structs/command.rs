use crate::structs;

pub struct Command {
    pub name: String,
    pub description: String,
    pub helpstring: String,
    pub execute: fn(bot: &structs::chatbot::ChatBot, args: &[&str]) -> String,
}

impl Command {
    pub fn new(name: &str, description: &str, helpstring: &str, execute: fn(bot: &structs::chatbot::ChatBot, args: &[&str]) -> String) -> Command {
        Command {
            name: name.to_owned(),
            description: description.to_owned(),
            helpstring: helpstring.to_owned(),
            execute,
        }
    }
}