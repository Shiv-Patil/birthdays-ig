use crate::structs;
use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};
use std::collections::HashMap;

pub struct ChatBot {
    pub commands: HashMap<String, structs::command::Command>,
    pub aliases: HashMap<String, String>,
    pub rl: Editor<()>,
}

impl ChatBot {
    pub fn new() -> ChatBot {
        ChatBot {
            commands: HashMap::new(),
            aliases: HashMap::new(),
            rl: Editor::<()>::new().unwrap(),
        }
    }

    pub fn register_command(&mut self, command: structs::command::Command) {
        for name in command.alias {
            let _ = self
                .aliases
                .insert(String::from(name.to_owned()), command.name.clone());
        }
        let _ = self
            .aliases
            .insert(command.name.clone(), command.name.clone());
        let _ = self.commands.insert(command.name.clone(), command);
    }

    pub fn run(&mut self) -> Result<()> {
        let exit_strings = [
            "q", "quit", "exit", "bye", "goodbye", "adios", "see ya", "so long", "stop", "gtg",
        ];
        println!("HI ^.^ type `help` to see all available commands.\ntype q to exit program.\n");
        loop {
            let readline = self.rl.readline("~> ");
            match readline {
                Ok(l) => {
                    let line = l.trim();
                    if line.is_empty() {
                        continue;
                    }
                    let mut parts = line.split_whitespace();
                    let name = parts.next().unwrap().to_owned().to_lowercase();
                    let args = parts.collect::<Vec<&str>>();

                    if exit_strings.iter().any(|&s| s == line.to_lowercase()) {
                        println!("\n:)\n");
                        break;
                    }

                    if &line.to_lowercase() == "rickroll" {
                        println!("\nNever gonna give you up\nNever gonna let you down\nNever gonna run around and desert you ^.^\n");
                        continue;
                    }

                    if !self.aliases.contains_key(&name) {
                        println!("No such command - `{}`\n", &name);
                        continue;
                    }

                    let command = self.commands.get(self.aliases.get(&name).unwrap()).unwrap();
                    let result = (command.execute)(self, &args);
                    println!("{result}");

                    let _ = self.rl.add_history_entry(line.trim());
                }
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    break;
                }
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    break;
                }
                Err(err) => {
                    println!("Error: {err:?}");
                    break;
                }
            }
        }
        Ok(())
    }
}
