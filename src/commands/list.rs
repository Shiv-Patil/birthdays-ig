use chrono::{Local, NaiveDate, Datelike};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read};
use crate::structs;

fn read_people() -> Result<HashMap<String, String>, std::io::Error> {
    let mut file = File::open("birthdays.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    if contents.is_empty() {
        return Ok(HashMap::new());
    }
    let people: HashMap<String, String> = serde_json::from_str(&contents)?;
    Ok(people)
}

fn equal_day_and_month(date1: &NaiveDate, date2: &NaiveDate) -> bool {
    if (date1.month() == date2.month()) && (date1.day() == date2.day()) {
        return true;
    }
    false
}

pub fn list_command(_bot: &structs::chatbot::ChatBot, args: &[&str]) -> String {
    let mut display_all = false;
    if args.len() >= 1 && &args[0].to_lowercase() == "all" {
        display_all = true;
    }

    let people = match read_people() {
        Ok(people) => people,
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
            Err(_e) => match NaiveDate::parse_from_str(&(day.to_owned()+"-2000"), "%0d-%0m-%Y") {
                Ok(bday) => bday,
                Err(_e) => {res_errors.push_str(&format!("{}: {}\n", person, day)); continue;}
            }
        };

        if equal_day_and_month(&birthday, &today) {
            res_today.push_str(&format!("{}: Today\n", person));
        } else if equal_day_and_month(&birthday, &today.succ_opt().unwrap()) {
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
