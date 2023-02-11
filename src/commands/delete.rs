use crate::{common, structs};
use std::fs::remove_file;
use std::io::ErrorKind;

pub fn get_command() -> structs::command::Command {
    let alias = &["remove", "erase"];
    structs::command::Command::new(
        "delete", alias,
        "Remove a birthday",
        &format!("takes 1 required argument <name>. Additional arguments <names> to delete can also be provided for bulk deletion.\n\
alias: {}", alias.join(", ")),
        delete_command
    )
}

fn delete_bithday(names: &[&str]) -> Result<String, String> {
    let mut people = match common::read_people() {
        Ok(p) => p,
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                return Ok("\nThere are no birthdays stored.\n".to_string());
            } else {
                return Err("The database file is corrupted. You can try to either fix birthdays.json or delete it and try again.".to_string());
            }
        }
    };

    let mut res = String::from("\n");
    let mut deleted = 0;

    for name in names {
        match people.remove(name.to_owned()) {
            Some(_) => {
                res.push_str(&format!("Successfully removed {name} from database.\n"));
                deleted += 1;
            }
            None => {
                res.push_str(&format!("`{name}` does not exist in database.\n"));
            }
        };
    }

    if people.is_empty() {
        match remove_file("birthdays.json") {
            Ok(_) => {}
            Err(_e) => {}
        };
    } else if deleted != 0 {
        common::write_people(&people)?;
    }
    if deleted != 0 {
        res.push_str(&format!(
            "\n{} {} deleted.\n",
            deleted,
            if deleted == 1 { "entry" } else { "entries" }
        ));
    } else {
        res.push_str("\nNo changes were made.\n");
    }

    Ok(res)
}

fn delete_command(_bot: &mut structs::chatbot::ChatBot, args: &[&str]) -> String {
    if args.is_empty() {
        return "\nCommand needs at least one argument <name>. Usage: delete <name1> <name2>[optional]...\n".to_owned();
    }

    match delete_bithday(args) {
        Ok(res) => res,
        Err(e) => format!("\nFailed to delete birthday: {e}\n"),
    }
}
