use crate::common;
use crate::structs;

pub fn get_command() -> structs::command::Command {
    let alias = &["date", "config"];
    structs::command::Command::new(
        "format", alias,
        "Change the format used for reading date",
        &format!("Change the format used for reading date from the database.\n\
Takes one required argument `format` and one optional argument `change` (default value = yes). format must be one of the following:\n\
1. dd-mm\n\
2. mm-dd\n\
The 2nd argument (yes, no) is used to specify whether to change the current dates in the database to match the new format selected.\n\
alias: {}", alias.join(", ")),
        format_command
    )
}

fn change_format(newfmt: String, change: bool) -> Result<String, String> {
    let (mut people, fmt) = match common::read_people() {
        Ok(p) => p,
        Err((e, _)) => return Err(e.to_string()),
    };

    if fmt == newfmt {
        return Ok("Bruv, you already using that format...".to_string());
    }

    if change {
        people = people
            .into_iter()
            .map(|(n, mut d)| {
                d.birthday = d
                    .birthday
                    .split('-')
                    .rev()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
                    .join("-");
                (n, d)
            })
            .collect();
    }

    common::write_people(&people, newfmt)?;

    Ok("Successfully changed format.".to_string())
}

fn format_command(_bot: &mut structs::chatbot::ChatBot, args: &[&str]) -> String {
    if args.len() > 2 || args.is_empty() {
        return "\nInvalid number of arguments. See `help format` for correct usage.\n".to_string();
    }

    let dd_mm = String::from("%0d-%0m");
    let mm_mm = String::from("%0m-%0d");

    let fmt = match args[0].to_lowercase().as_str() {
        "1" => dd_mm,
        "2" => mm_mm,
        "dd-mm" => dd_mm,
        "mm-dd" => mm_mm,
        "d-m" => dd_mm,
        "m-d" => mm_mm,
        _ => {
            return "\nInvalid `format` argument. See `help format` for more details\n".to_string()
        }
    };

    let change = if args.len() == 2 {
        args[1].to_lowercase() != "no"
    } else {
        true
    };

    match change_format(fmt, change) {
        Ok(res) => format!("\n{res}\n"),
        Err(e) => format!("\nError: {e}\n"),
    }
}
