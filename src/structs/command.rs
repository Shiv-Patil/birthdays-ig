use crate::structs;

pub struct Command {
    pub name: String,
    pub alias: &'static [&'static str],
    pub description: String,
    pub helpstring: String,
    pub execute: fn(bot: &mut structs::chatbot::ChatBot, args: &[&str]) -> String,
}

impl Command {
    pub fn new(
        name: &str,
        alias: &'static [&'static str],
        description: &str,
        helpstring: &str,
        execute: fn(bot: &mut structs::chatbot::ChatBot, args: &[&str]) -> String,
    ) -> Command {
        Command {
            name: name.to_owned(),
            alias,
            description: description.to_owned(),
            helpstring: helpstring.to_owned(),
            execute,
        }
    }
}
