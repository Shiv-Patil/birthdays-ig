use std::collections::HashSet;
use chrono::{Local, NaiveDate};
use std::io::ErrorKind;
use crate::structs;
use crate::common;

pub fn get_command() -> structs::command::Command {
    let alias = &["fetch", "who", "when"];
    structs::command::Command::new(
        "find", alias,
        "Get name and birthday of a perticular person",
        &format!("If name is given - Fetches the birthday of the person given the name.\n\
If date is given - Fetches the names of all the people having their birthday on the given date.\n\
alias: {}", alias.join(", ")),
        fetch_command
    )
}

fn fetch_command(_bot: &structs::chatbot::ChatBot, args: &[&str]) -> String {
    if args.len() == 0 {
        return "\nThis command requires at least 1 argument - either a <name> or a <date>.\n".to_string();
    }

    let mut res = String::from("\n");
    let mut matches = 0;
    let people = match common::read_people() {
        Ok(people) => people,
        Err(ref e) if e.kind() == ErrorKind::InvalidData => return format!("\nFailed to read database: {}\n", e),
        Err(e) => return format!("\nFailed to read database: {}\nIt is possible that the database file doesn't exist. Add a new birthday to be able to find birthdays.\n", e),
    };

    let mut already_added: HashSet<String> = HashSet::new();
    for arg in args {
        let mut is_date = false;
        let date = match NaiveDate::parse_from_str(arg, "%0d-%0m-%Y") {
            Ok(d) => {is_date = true; d},
            Err(_e) => match NaiveDate::parse_from_str(&format!("{}-2001", arg), "%0d-%0m-%Y") {
                Ok(d) => {is_date = true; d},
                Err(_e) => Local::now().date_naive()
            }
        };

        for (name, birthday) in &people {
            if already_added.contains(name) {
                continue;
            }
            if is_date {
                let bday = match NaiveDate::parse_from_str(birthday, "%0d-%0m-%Y") {
                    Ok(d) => d,
                    Err(_e) => match NaiveDate::parse_from_str(&format!("{}-2001", birthday), "%0d-%0m-%Y") {
                        Ok(d) => d,
                        Err(_e) => continue
                    }
                };
                if common::equal_day_and_month(&bday, &date) {
                    matches += 1;
                    res.push_str(&format!("{}: {}\n", name, birthday));
                    already_added.insert(name.clone());
                }
            } else if arg == name {
                matches += 1;
                res.push_str(&format!("{}: {}\n", name, birthday));
                already_added.insert(name.clone());
            }
        }
    }
    if matches == 0 {
        res.push_str("No match found\n");
    } else {
        res.push_str(&format!("\n{} {} found\n", matches, if matches == 1 {"match"} else {"matches"}));
    }
    res
}
