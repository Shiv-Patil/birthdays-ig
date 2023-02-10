use crate::common;
use crate::structs;
use chrono::Local;
use rand::seq::IteratorRandom;

pub fn get_command() -> structs::command::Command {
    let alias = &["question", "ask", "test"];
    structs::command::Command::new(
        "quiz", alias,
        "Ask the birthday of a random person to test if you know their birthday",
        &format!("This command will choose a random person from the saved birthdays.\n\
you have to input the birthday of the person based on your memory. This will check if you know your birthdays :)\n\
alias: {}", alias.join(", ")),
        quiz_command
    )
}

fn quiz_command(bot: &mut structs::chatbot::ChatBot, _args: &[&str]) -> String {
    let mut people = match common::read_people() {
        Ok(people) => people,
        Err(e) => return format!("\nError: {e}\n"),
    };

    let mut chosen = false;
    let mut person = String::from("Adonis");
    let mut bday = Local::now().date_naive();
    while !people.is_empty() {
        let peopleiter = people.clone();
        let item = peopleiter.iter().choose(&mut rand::thread_rng()).unwrap();
        person = item.0.to_string();
        bday = match common::parse_birthday(item.1) {
            Ok(d) => {
                chosen = true;
                d
            }
            Err(_e) => {
                let _ = people.remove(&person);
                continue;
            }
        };
        break;
    }
    if !chosen {
        return "\nError: There are no birthdays in the correct format stored.\n".to_string();
    }

    let hist: Vec<String> = bot.rl.history().iter().cloned().collect();
    bot.rl.history_mut().clear();
    let res = loop {
        let readline = bot.rl.readline(&format!("\nWhat is {person}'s birthday: "));
        match readline {
            Ok(l) => {
                let line = l.trim();
                if line.is_empty() {
                    break "Quiz cancelled.\n".to_string();
                }
                let bday_input = match common::parse_birthday(line) {
                    Ok(d) => d,
                    Err(_e) => {
                        println!("Please enter date correctly (dd-mm or dd-mm-yyyy)");
                        continue;
                    }
                };
                if common::equal_day_and_month(&bday, &bday_input) {
                    break format!("\nAwesome, You remember {person}'s birthday!\n");
                } else {
                    break format!(
                        "\nWrong answer lol. skill issue.\n\
The correct ans is {}\n",
                        bday.format("%B %d")
                    );
                }
            }
            Err(_err) => {
                break "Quiz cancelled.\n".to_string();
            }
        }
    };
    for line in hist {
        let _ = bot.rl.history_mut().add(line);
    }
    res
}
