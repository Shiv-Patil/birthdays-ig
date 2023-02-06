use serde_json;
use std::fs::File;
use std::io::{Read, Write};
use std::collections::HashMap;
use crate::structs;

fn add_person(name: &str, birthday: &str) -> Result<String, std::io::Error> {

    let mut contents = String::new();
    match File::open("birthdays.json") {
        Ok(mut readfile) => {readfile.read_to_string(&mut contents)?;},
        Err(_e) => {}
    };

    let mut savefile = File::create("birthdays.json").unwrap();

    let mut people: HashMap<String, String> = if contents.is_empty() {HashMap::new()} else {serde_json::from_str(&contents)?};
    let updated = people.insert(name.to_owned(), birthday.to_owned());
    let serialized = serde_json::to_string(&people)?;

    write!(savefile, "{}", serialized)?;

    match updated {
        Some(old) => return Ok(format!("\nSuccessfully updated the birthday of {} from {} to {}.\n", name, old, birthday)),
        None => return Ok("\nSuccessfully added person to the file.\n".to_owned())
    };
}

// Define the add command for the chat bot
pub fn add_command(_bot: &structs::chatbot::ChatBot, args: &[&str]) -> String {
    // Check that there are exactly two arguments
    if args.len() != 2 {
        return "\nInvalid number of arguments. Usage: add <name> <birthday>\n".to_owned();
    }

    // Call the add_person function with the two arguments
    match add_person(args[0], args[1]) {
        Ok(res) => res,
        Err(e) => format!("\nFailed to add person: {}\n", e),
    }
}
