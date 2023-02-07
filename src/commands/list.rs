use chrono::{Local, NaiveDate};
use std::io::ErrorKind;
use crate::structs;
use crate::common;

pub fn get_command() -> structs::command::Command {
    let alias = &["show", "upcoming", "next"];
    structs::command::Command::new(
        "list", alias,
        "List the upcoming birthdays",
        &format!("Lists the people and their birthdays whose birthdays are due today or tomorrow.\nUse with the argument `all` to display all birthdays.\n\
alias: {}", alias.join(", ")),
        list_command
    )
}

fn list_command(_bot: &structs::chatbot::ChatBot, args: &[&str]) -> String {
    let mut display_all = false;
    if args.len() >= 1 && &args[0].to_lowercase() == "all" {
        display_all = true;
    }

    let people = match common::read_people() {
        Ok(people) => people,
        Err(ref e) if e.kind() == ErrorKind::InvalidData => return format!("\nFailed to read database: {}\n", e),
        Err(e) => return format!("\nFailed to read database: {}\nIt is possible that the database file doesn't exist. Add a new birthday to be able to list birthdays.\n", e),
    };

    let today = Local::now().date_naive();
    let mut res_today = String::new();
    let mut res_tomorrow = String::new();
    let mut res_later = String::new();
    let mut res_errors = String::new();

    for (person, day) in people {
        let birthday = match NaiveDate::parse_from_str(&day, "%0d-%0m-%Y") {
            Ok(bday) => bday,
            Err(_e) => match NaiveDate::parse_from_str(&(day.to_owned()+"-2001"), "%0d-%0m-%Y") {
                Ok(bday) => bday,
                Err(_e) => {res_errors.push_str(&format!("{}: {}\n", person, day)); continue;}
            }
        };

        if common::equal_day_and_month(&birthday, &today) {
            res_today.push_str(&format!("{}: Today\n", person));
        } else if common::equal_day_and_month(&birthday, &today.succ_opt().unwrap()) {
            res_tomorrow.push_str(&format!("{}: Tomorrow\n", person));
        } else {
            res_later.push_str(&format!("{}: {}\n", person, birthday.format("%B %d")));
        }
    }
    let mut result = String::new();
    if !res_today.is_empty() {
        result.push_str("\n");
        result.push_str(&res_today);
    }
    if !res_tomorrow.is_empty() {
        result.push_str("\n");
        result.push_str(&res_tomorrow);
    }
    if display_all && !res_later.is_empty() {
        result.push_str("\n");
        result.push_str(&res_later);
    }
    if !res_errors.is_empty() {
        result.push_str(&format!("\nThere were some errors found in the database file, which are given below. These are possibly due to the dates being in the wrong format or invalid.\n\
Please update them to the correct format (either `dd-mm-yyyy` or `dd-mm`) by using the add command.\n\n{}", res_errors));
    }

    if result.is_empty() {
        "No upcoming birthdays.".to_owned()
    } else {
        result
    }
}
