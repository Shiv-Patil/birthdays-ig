use crate::common;
use crate::structs;
use chrono::Local;

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

fn list_command(_bot: &mut structs::chatbot::ChatBot, args: &[&str]) -> String {
    let mut display_all = false;
    if !args.is_empty() && &args[0].to_lowercase() == "all" {
        display_all = true;
    }

    let peoplehash = match common::read_people() {
        Ok(people) => people,
        Err(e) => return format!("\nError: {e}\n"),
    };
    let mut people: Vec<(String, String)> = peoplehash.into_iter().collect();
    people.sort_by(|a, b| a.0.cmp(&b.0));

    let today = Local::now().date_naive();
    let mut res_today = String::new();
    let mut res_tomorrow = String::new();
    let mut res_later = String::new();
    let mut res_errors = String::new();

    for (person, day) in people {
        let birthday = match common::parse_birthday(&day) {
            Ok(d) => d,
            Err(_e) => {
                res_errors.push_str(&format!("{person}: {day}\n"));
                continue;
            }
        };

        if common::equal_day_and_month(&birthday, &today) {
            res_today.push_str(&format!("{person}: Today\n"));
        } else if common::equal_day_and_month(&birthday, &today.succ_opt().unwrap()) {
            res_tomorrow.push_str(&format!("{person}: Tomorrow\n"));
        } else {
            res_later.push_str(&format!("{}: {}\n", person, birthday.format("%B %d")));
        }
    }
    let mut result = String::new();
    if !res_today.is_empty() {
        result.push('\n');
        result.push_str(&res_today);
    }
    if !res_tomorrow.is_empty() {
        result.push('\n');
        result.push_str(&res_tomorrow);
    }
    if display_all && !res_later.is_empty() {
        result.push('\n');
        result.push_str(&res_later);
    }
    if !res_errors.is_empty() {
        result.push_str(&format!("\nThere were some errors found in the database file, which are given below. These are possibly due to the dates being in the wrong format or invalid.\n\
Please update them to the correct format (either `dd-mm-yyyy` or `dd-mm`) by using the add command.\n\n{res_errors}"));
    }

    if result.is_empty() {
        "\nNo upcoming birthdays.\n\
If you are looking for all birthdays, use the command again with the `all` argument (list all).\n"
            .to_owned()
    } else {
        result
    }
}
