use serde_json;
use std::fs::{File, remove_file};
use std::io::{Read, Write};
use std::collections::HashMap;
use crate::structs;

fn delete_bithday(names: &[&str]) -> Result<String, std::io::Error> {

    let mut file = File::open("birthdays.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut people: HashMap<String, String> = if contents.is_empty() {HashMap::new()} else {serde_json::from_str(&contents)?};
    let mut savefile = File::create("birthdays.json").unwrap();

    let mut res = String::from("\n");
    let mut deleted = 0;

    for name in names {
        match people.remove(name.to_owned()) {
            Some(removed) => {
                res.push_str(&format!("Successfully removed the birthday of {} ({}).\n", name, removed));
                deleted += 1;
            },
            None => {res.push_str(&format!("No birthday for `{}` exists.\n", name));}
        };
    }

    if people.is_empty() {
        match remove_file("birthdays.json") {
            Ok(_) => {},
            Err(_e) => {}
        };
    } else if deleted != 0 {
        let serialized = serde_json::to_string(&people)?;
        write!(savefile, "{}", serialized)?;
    }
    if deleted != 0 {
        res.push_str(&format!("\n{} {} deleted.\n", deleted, if deleted == 1 {"entry"} else {"entries"}));
    } else {
        res.push_str(&format!("\nNo changes were made.\n"));
    }

    Ok(res)
}

pub fn delete_command(_bot: &structs::chatbot::ChatBot, args: &[&str]) -> String {
    // Check that there is exactly one arguments
    if args.len() == 0 {
        return "\nCommand needs at least one argument <name>. Usage: delete <name1> <name2>[optional]...\n".to_owned();
    }

    match delete_bithday(args) {
        Ok(res) => res,
        Err(e) => format!("\nFailed to delete birthday: {}\nIt is possible that the database file doesn't exist. Add a new birthday to be able to remove birthdays.\n", e),
    }
}
