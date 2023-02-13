use crate::common;
use crate::structs;
use csv;
use std::{collections::HashMap, fs::File, io::BufReader, io::ErrorKind};

pub fn get_command() -> structs::command::Command {
    let alias = &["read", "pull"];
    structs::command::Command::new(
        "import",
        alias,
        "Import birthdays from csv file",
        &format!(
            "Import birthdays in bulk from a csv file. The data format should be:\n\
<name>, <birthday>: one entry on each line.\n\
Usage - `import <path-to-csv-file>` (eg. import ../birthdays.csv) \n\
alias: {}",
            alias.join(", ")
        ),
        import_command,
    )
}

fn import_command(_bot: &mut structs::chatbot::ChatBot, args: &[&str]) -> String {
    if args.is_empty() {
        return "\nPlease provide the file to import from as an argument to the command.\n\
Run `help import` for more details.\n"
            .to_string();
    }

    match read_csv(args[0]) {
        Ok(s) => s,
        Err(e) => format!("\nError: {e}\n"),
    }
}

fn read_csv(path: &str) -> Result<String, String> {
    let file = match File::open(path) {
        Ok(r) => r,
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                return Err("No file found with the given path".to_string());
            } else if e.kind() == ErrorKind::InvalidInput {
                return Err("Given path is not valid".to_string());
            } else {
                return Err("Unknown error trying to open file".to_string());
            }
        }
    };
    
    let file_len = file.metadata().map_err(|e| e.to_string())?.len();
    
    if file_len == 0 {
        return Err("File empty".to_string());
    }
    
    let reader = BufReader::new(file);
    let mut csvreader = csv::ReaderBuilder::new().has_headers(false).from_reader(reader);
    
    let (mut people, fmt) = match common::read_people() {
        Ok(p) => p,
        Err((e, fmt)) => {
            if e.kind() == ErrorKind::NotFound {
                (HashMap::new(), fmt)
            } else {
                return Err("The database file is corrupted. You can try to either fix birthdays.json or delete it and try again.".to_string());
            }
        }
    };
    let mut errors = String::new();

    for res in csvreader.records() {
        let record = res.map_err(|e| e.to_string())?;
        if record.len() != 2 {
            return Err("Please make sure the csv file is in the right format.".to_string());
        }
        let name = record[0].trim().to_string();
        let birthday = record[1].trim().to_string();
        match common::parse_birthday(&birthday, &fmt) {
            Ok(_d) => (),
            Err(_) => {
                errors.push_str(&format!("{name}: {birthday}\n"));
                continue;
            }
        };
        let _ = people.insert(
            name,
            structs::person::Person {
                birthday,
                fields: HashMap::new(),
            },
        );
    }

    if people.is_empty() {
        return Err("The data in CSV file is not in the right format.\n\
Please use the format `dd-mm` or `dd-mm-yyyy` for birthdays."
            .to_string());
    }

    common::write_people(&people, fmt)?;

    let mut res = String::from("\nSuccessfully imported CSV file.\n");
    if !errors.is_empty() {
        res.push_str("\nSome error were found in the CSV file most likely due to the dates being in the wrong format.\n\
These are as follows:\n");
        res.push_str(&errors);
    }

    Ok(res)
}
