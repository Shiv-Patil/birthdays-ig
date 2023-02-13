use crate::common;
use crate::structs;
use csv;
use std::{collections::HashMap, fs::File, io::ErrorKind, path::Path};

pub fn get_command() -> structs::command::Command {
    let alias = &["write", "backup", "push"];
    structs::command::Command::new(
        "export", alias,
        "Export birthdays to a csv file",
        &format!("Export all birthdays in database to a csv file. The data format will remain the same as stored.\n\
Usage - `export <filename-to-save-as>` (eg. export birthdays.csv) \n\
alias: {}", alias.join(", ")),
        export_command
    )
}

fn export_command(_bot: &mut structs::chatbot::ChatBot, args: &[&str]) -> String {
    if args.is_empty() {
        return "\nPlease provide the filename for the csv file as an argument to the command.\n\
Run `help export` for more details.\n"
            .to_string();
    }

    match write_csv(args[0].to_string()) {
        Ok(s) => s,
        Err(e) => format!("\nError: {e}\n"),
    }
}

fn write_csv(mut name: String) -> Result<String, String> {
    if !name.ends_with(".csv") {
        name = format!("{name}.csv")
    }
    let path = Path::new(&name);
    if path.exists() {
        return Err("A file with that filename already exists.".to_string());
    }

    let (people, _fmt) = match common::read_people() {
        Ok(p) => p,
        Err((e, fmt)) => {
            if e.kind() == ErrorKind::NotFound {
                (HashMap::new(), fmt)
            } else {
                return Err(
                    "The database file is corrupted. You can try to fix birthdays.json try again."
                        .to_string(),
                );
            }
        }
    };

    let savefile = File::create(path).map_err(|e| e.to_string())?;
    let mut writer = csv::Writer::from_writer(savefile);

    for (name, person) in people.iter() {
        writer
            .write_record([name, &person.birthday])
            .map_err(|e| e.to_string())?;
    }

    writer.flush().map_err(|e| e.to_string())?;
    Ok(format!(
        "\nSuccessfully exported CSV as {}\n",
        path.to_string_lossy()
    ))
}
