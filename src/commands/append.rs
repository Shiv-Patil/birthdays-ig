use crate::common;
use crate::structs;
use std::collections::HashMap;
use std::io::ErrorKind;

pub fn get_command() -> structs::command::Command {
    let alias = &["append", "insert", "new", "edit"];
    structs::command::Command::new(
        "add",
        alias,
        "Add a birthday",
        &format!(
            "Takes 2 arguments (name and birthday). Adds the entry to the database.\n\
alias: {}",
            alias.join(", ")
        ),
        add_command,
    )
}

fn add_person(name: &str, birthday: &str) -> Result<String, String> {
    let mut people: HashMap<String, String> = match common::read_people() {
        Ok(p) => p,
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                HashMap::new()
            } else {
                return Err("The database file is corrupted. You can try to either fix birthdays.json or delete it and try again.".to_string());
            }
        }
    };

    let updated = people.insert(name.to_owned(), birthday.to_owned());
    common::write_people(&people)?;

    match updated {
        Some(old) => Ok(format!(
            "\nSuccessfully updated the birthday of {name} from {old} to {birthday}.\n"
        )),
        None => Ok("\nSuccessfully added person to the file.\n".to_owned()),
    }
}

fn add_command(_bot: &mut structs::chatbot::ChatBot, args: &[&str]) -> String {
    if args.len() != 2 {
        return "\nInvalid number of arguments. Usage: add <name> <birthday>\n".to_owned();
    }

    match add_person(args[0], args[1]) {
        Ok(res) => res,
        Err(e) => format!("\nFailed to add person: {e}\n"),
    }
}
