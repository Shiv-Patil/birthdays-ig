use crate::common;
use crate::structs;
use csv;
use csv::StringRecord;
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
    let mut csvreader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(reader);

    let (mut people, fmt) = match common::read_people() {
        Ok(p) => p,
        Err((e, fmt)) => {
            if e.kind() == ErrorKind::NotFound {
                (HashMap::new(), fmt)
            } else {
                return Err(e.to_string());
            }
        }
    };
    let mut errors = String::new();
    let mut records = csvreader.records();

    let headers = match records.next() {
        None => return Err("CSV file is empty".to_string()),
        Some(r) => r.map_err(|e| e.to_string())?,
    };
    let mut entries: Vec<StringRecord> = records
        .filter_map(|r| match r {
            Ok(r) => Some(r),
            Err(_) => None,
        })
        .collect();

    if headers.len() < 2 {
        return Err("Please make sure the csv file is in the right format.".to_string());
    }

    let header = headers[0]
        .split('=')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let mut newfmt = fmt.to_string();
    if header[0] == "format" && (header[1] == "%0d-%0m" || header[1] == "%0m-%0d") {
        newfmt = header[1].to_string();
    } else {
        entries.push(headers);
        eprintln!("\nFormat not specified in csv. Defaulting to current format.");
    }

    for record in entries {
        let name = record[0].trim().to_string();
        let mut birthday = record[1].trim().to_string();
        match common::parse_birthday(&birthday, &newfmt) {
            Ok(_) => {
                if newfmt != fmt {
                    birthday = common::change_format(birthday)
                }
            }
            Err(_) => {
                errors.push_str(&format!("{name}: {birthday}\n"));
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

    common::write_people(&people, fmt)?;

    let mut res = String::from("\nSuccessfully imported CSV file.\n");
    if !errors.is_empty() {
        res.push_str("\nSome error were found in the CSV file most likely due to the dates being in the wrong format.\n\
These are as follows:\n");
        res.push_str(&errors);
    }

    Ok(res)
}
