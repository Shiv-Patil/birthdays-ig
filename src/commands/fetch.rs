use crate::common;
use crate::structs;
use chrono::Local;
use std::collections::HashSet;

pub fn get_command() -> structs::command::Command {
    let alias = &["fetch", "who", "when"];
    structs::command::Command::new(
        "find",
        alias,
        "Get name and birthday of a particular person",
        &format!(
            "If name is given - Fetches the birthday of the person given the name.\n\
If date is given - Fetches the names of all the people having their birthday on the given date.\n\
alias: {}",
            alias.join(", ")
        ),
        fetch_command,
    )
}

fn fetch_command(_bot: &mut structs::chatbot::ChatBot, args: &[&str]) -> String {
    if args.is_empty() {
        return "\nThis command requires at least 1 argument - either a <name> or a <date>.\n"
            .to_string();
    }

    let mut res = String::from("\n");
    let mut res_error = String::new();
    let mut matches = 0;
    let people = match common::read_people() {
        Ok(people) => people,
        Err(e) => return format!("\nError: {e}\n"),
    };

    let mut already_added: HashSet<String> = HashSet::new();
    for arg in args {
        let mut is_date = false;
        let date = match common::parse_birthday(arg) {
            Ok(d) => {
                is_date = true;
                d
            }
            Err(_e) => Local::now().date_naive(),
        };

        for (name, person) in &people {
            if already_added.contains(name) {
                continue;
            }
            _ = already_added.insert(name.clone());
            if is_date {
                let bday = match common::parse_birthday(&person.birthday) {
                    Ok(d) => d,
                    Err(_e) => {
                        res_error.push_str(&format!("{name}: {}\n", person.birthday));
                        continue;
                    }
                };
                if common::equal_day_and_month(&bday, &date) {
                    matches += 1;
                    res.push_str(&format!("{name}: {}\n", person.birthday));
                }
            } else if arg == name {
                matches += 1;
                let bday = if person.birthday.is_empty() {
                    "No birthday stored"
                } else {
                    &person.birthday
                };
                res.push_str(&format!("{name}: {bday}\n"));
            }
        }
    }
    if matches == 0 {
        res.push_str("No match found\n");
    } else {
        res.push_str(&format!(
            "\n{} {} found\n",
            matches,
            if matches == 1 { "match" } else { "matches" }
        ));
    }
    if !res_error.is_empty() {
        res.push_str(&format!("\nInvalid birthdays stored:\n{res_error}"));
    }
    res
}
