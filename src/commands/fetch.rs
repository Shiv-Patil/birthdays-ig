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

    let mut res = String::new();
    let mut res_error = String::new();
    let mut matches = 0;

    let (people, fmt) = match common::read_people() {
        Ok(p) => p,
        Err((e, _)) => return format!("\nError: {e}\n"),
    };

    let mut already_added: HashSet<String> = HashSet::new();
    for arg in args {
        let mut is_date = false;
        let date = match common::parse_birthday(arg, &fmt) {
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
            if arg == name {
                _ = already_added.insert(name.clone());
            }
            if is_date {
                let bday = match common::parse_birthday(&person.birthday, &fmt) {
                    Ok(d) => d,
                    Err(_e) => continue,
                };
                if common::equal_day_and_month(&bday, &date) {
                    matches += 1;
                    res.push_str(&format!("\n{name}: {}", person.birthday));
                }
            } else if arg == name {
                let bday = if person.birthday.is_empty() {
                    matches += 1;
                    "No birthday stored"
                } else {
                    match common::parse_birthday(&person.birthday, &fmt) {
                        Ok(_) => (),
                        Err(_e) => {
                            res_error.push_str(&format!("\n{name}: {}", person.birthday));
                            continue;
                        }
                    }
                    matches += 1;
                    &person.birthday
                };
                res.push_str(&format!("\n{name}: {bday}"));
                if args.len() == 1 {
                    let mut sorted: Vec<(&String, &String)> = person.fields.iter().collect();
                    sorted.sort_by(|a, b| a.0.cmp(b.0));
                    res.push('\n');
                    for (field, content) in sorted {
                        res.push_str(&format!("\n{field} - {content}\n"));
                    }
                }
            }
        }
    }

    if matches == 0 {
        res.push_str("\nNo match found\n");
    } else if matches > 1 {
        res.push_str(&format!("\n\n{matches} matches found\n",));
    }

    if !res_error.is_empty() {
        res.push_str(&format!("\nInvalid birthdays stored:\n{res_error}\n"));
    }
    res
}
