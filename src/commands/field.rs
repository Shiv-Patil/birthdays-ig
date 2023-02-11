use crate::common;
use crate::structs;
use rustyline::{Cmd, EventHandler, KeyCode, KeyEvent, Modifiers};

pub fn get_command() -> structs::command::Command {
    let alias = &["attribute", "attr", "custom", "store"];
    structs::command::Command::new(
        "field",
        alias,
        "Add a custom field to an entry",
        &format!(
            "Takes 2 arguments (name and field). Creates a new custom field for the particular entry.\n\
This can be used to store additional information about a person.\n\
Usage: `field [add|delete] <name> <field>`\n\
alias: {}",
            alias.join(", ")
        ),
        field_command,
    )
}

fn add_field(
    bot: &mut structs::chatbot::ChatBot,
    name: &str,
    field: &str,
) -> Result<String, String> {
    let mut people = match common::read_people() {
        Ok(p) => p,
        Err(e) => return Err(e.to_string()),
    };

    let mut entry = match people.remove(name) {
        None => return Err(format!("`{name}` does not exist in database.")),
        Some(e) => e,
    };

    let hist: Vec<String> = bot.rl.history().iter().cloned().collect();
    bot.rl.history_mut().clear();
    _ = bot.rl.bind_sequence(
        KeyEvent(KeyCode::Char('n'), Modifiers::CTRL),
        EventHandler::Simple(Cmd::Newline),
    );

    let res = (|| {
        let readline = bot
            .rl
            .readline("\nField content (`CTRL + n` for newline): ");
        match readline {
            Ok(l) => {
                let line = l.trim().to_string();
                if line.is_empty() {
                    return "Cancel.\n".to_string();
                }
                match entry.fields.insert(field.to_string(), line) {
                    None => format!("\nSuccessfully added field `{field}`\n"),
                    Some(_) => format!("\nSuccessfully updated field `{field}`\n"),
                }
            }
            Err(_err) => "Cancel.\n".to_string(),
        }
    })();

    _ = bot
        .rl
        .unbind_sequence(KeyEvent(KeyCode::Char('n'), Modifiers::CTRL));

    for line in hist {
        let _ = bot.rl.history_mut().add(line);
    }

    _ = people.insert(name.to_string(), entry);

    common::write_people(&people)?;

    Ok(res)
}

fn delete_field(name: &str, field: &str) -> String {
    let mut people = match common::read_people() {
        Ok(p) => p,
        Err(e) => return format!("\nError: {e}\n"),
    };

    let mut entry = match people.remove(name) {
        None => return format!("\n`{name}` does not exist in database.\n"),
        Some(e) => e,
    };

    match entry.fields.remove(field) {
        None => {
            format!("\nThere is no field `{field}` stored for `{name}`. No changes were made.\n")
        }
        Some(_) => {
            _ = people.insert(name.to_string(), entry);
            match common::write_people(&people) {
                Ok(()) => format!("\nDeleted field `{field}` for `{name}`.\n"),
                Err(e) => format!("\nError: Writing changes to database failed: {e}\n"),
            }
        }
    }
}

fn field_command(bot: &mut structs::chatbot::ChatBot, args: &[&str]) -> String {
    if args.len() != 3 {
        return "\nInvalid number of arguments. See `help field` for correct usage.\n".to_owned();
    }

    if args[0].to_lowercase() == "add" {
        match add_field(bot, args[1], args[2]) {
            Ok(res) => res,
            Err(e) => format!("\nFailed to add field: {e}\n"),
        }
    } else if args[0].to_lowercase() == "delete" {
        delete_field(args[1], args[2])
    } else {
        "\nInvalid option specified. Must be one of `add` or `delete`.\n".to_string()
    }
}
