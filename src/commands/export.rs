use std::{path::Path, fs::File, io::ErrorKind, collections::HashMap};
use csv;
use crate::structs;
use crate::common;

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
    if args.len() == 0 {
        return "\nPlease provide the filename for the csv file as an argument to the command.\n\
Run `help export` for more details.\n".to_string();
    }

    match write_csv(args[0].to_string()) {
        Ok(s) => s,
        Err(e) => format!("\nError: {}\n", e)
    }
}

fn write_csv(mut name: String) -> Result<String, String> {
    if !name.ends_with(".csv") {
        name = format!("{}.csv", name)
    }
    let path = Path::new(&name);
    if path.exists() {
        return Err("A file with that filename already exists.".to_string());
    }

    let people = match common::read_people() {
        Ok(p) => p,
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                HashMap::new()
            } else {
                return Err("The database file is corrupted. You can try to fix birthdays.json try again.".to_string());
            }
        }
    };

    let savefile = File::create(path).map_err(|e| e.to_string())?;
    let mut writer = csv::Writer::from_writer(savefile);

    for (name, birthday) in people.iter() {
        writer.write_record(&[name, birthday]).map_err(|e| e.to_string())?;
    }

    writer.flush().map_err(|e| e.to_string())?;
    Ok(format!("\nSuccessfully exported CSV as {}\n", path.to_string_lossy()))
}
