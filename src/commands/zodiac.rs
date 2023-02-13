use crate::common;
use crate::structs;
use chrono::{Datelike, Local, NaiveDate};
use std::collections::HashSet;

pub fn get_command() -> structs::command::Command {
    let alias = &["sign", "symbol"];
    structs::command::Command::new(
        "zodiac",
        alias,
        "Get the zodiac sign of a person",
        &format!(
            "Get the zodiac sign based on the birthday.\n\
This command takes one required argument (name or birthday) upto any number of optional arguments.\n\
alias: {}",
            alias.join(", ")
        ),
        zodiac_command,
    )
}

fn get_zodiac_sign(birthday: &NaiveDate) -> &str {
    let day = birthday.day();
    let month = birthday.month();

    match (month, day) {
        (3, 21..=31) | (4, 1..=19) => "Aries",
        (4, 20..=30) | (5, 1..=20) => "Taurus",
        (5, 21..=31) | (6, 1..=20) => "Gemini",
        (6, 21..=30) | (7, 1..=22) => "Cancer",
        (7, 23..=31) | (8, 1..=22) => "Leo",
        (8, 23..=31) | (9, 1..=22) => "Virgo",
        (9, 23..=30) | (10, 1..=22) => "Libra",
        (10, 23..=31) | (11, 1..=21) => "Scorpio",
        (11, 22..=30) | (12, 1..=21) => "Sagittarius",
        (12, 22..=31) | (1, 1..=19) => "Capricorn",
        (1, 20..=31) | (2, 1..=18) => "Aquarius",
        (2, 19..=29) | (3, 1..=20) => "Pisces",
        _ => "Invalid input",
    }
}

fn zodiac_command(_bot: &mut structs::chatbot::ChatBot, args: &[&str]) -> String {
    if args.is_empty() {
        return "\nPlease pass in either name or birthday as an argument.\n\
Run `help zodiac` for more details.\n"
            .to_string();
    }

    let mut res = String::from("\n");
    let (people, fmt) = match common::read_people() {
        Ok(people) => people,
        Err((e, _)) => return format!("\nError: {e}\n"),
    };

    let mut already_added: HashSet<String> = HashSet::new();
    for arg in args {
        if already_added.contains(&arg.to_string()) {
            continue;
        }
        _ = already_added.insert(arg.to_string());

        let mut is_date = false;
        let date = match common::parse_birthday(arg, &fmt) {
            Ok(d) => {
                is_date = true;
                d
            }
            Err(_e) => Local::now().date_naive(),
        };

        if is_date {
            let date_formatted = date.format("%B %d");
            let sign = get_zodiac_sign(&date);
            res.push_str(&format!("{date_formatted}: {sign}\n"));
        } else {
            let person = match people.get(&arg.to_string()) {
                None => {
                    res.push_str(&format!("{arg}: Not found / Invalid date\n"));
                    continue;
                },
                Some(d) => d,
            };
            if person.birthday.is_empty() {
                res.push_str(&format!("{arg}: No birthday stored\n"));
                continue;
            }
            let date = match common::parse_birthday(&person.birthday, &fmt) {
                Ok(d) => d,
                Err(_e) => {
                    res.push_str(&format!("{arg}: Invalid birthday value\n"));
                    continue;
                }
            };
            let date_formatted = date.format("%B %d");
            let sign = get_zodiac_sign(&date);
            res.push_str(&format!("{arg} ({date_formatted}): {sign}\n"));
        }
    }

    res
}
