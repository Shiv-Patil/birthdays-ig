use crate::structs::person::Person;
use chrono::{Datelike, NaiveDate};
use serde_json::{from_str, Map, Value};
use std::collections::HashMap;
use std::fs::File;
use std::io::{stdout, Error, ErrorKind, Read, Write};

pub fn read_people() -> Result<(HashMap<String, Person>, String), (std::io::Error, String)> {
    let default_fmt = "%0d-%0m".to_string();

    let mut file = match File::open("birthdays.json") {
        Ok(f) => f,
        Err(ref e) if e.kind() == ErrorKind::NotFound => {
            return Err((
                Error::new(
                    ErrorKind::NotFound,
                    "The database file was not found. No birthday is stored.",
                ),
                default_fmt,
            ))
        }
        Err(e) => return Err((e, default_fmt)),
    };
    let mut contents = String::new();
    let _ = file
        .read_to_string(&mut contents)
        .map_err(|e| (e, default_fmt.clone()))?;
    if contents.is_empty() {
        return Ok((HashMap::new(), default_fmt));
    }
    let err_corrupt = |s| Error::new(ErrorKind::InvalidData, s);
    let database: Value = match from_str(&contents) {
        Ok(p) => p,
        Err(_e) => {
            return Err((
                err_corrupt("Corrupted database file, error trying to parse json."),
                default_fmt,
            ))
        }
    };

    let format_pref = match database.get("format") {
        None => "%0d-%0m",
        Some(s) => match s.as_str() {
            None => "%0d-%0m",
            Some(f) => match (|| {
                if f.to_lowercase() == "%0d-%0m" || f.to_lowercase() == "%0m-%0d" {
                    return Ok(());
                }
                Err(())
            })() {
                Ok(_) => f,
                Err(_) => {
                    println!("\nInvalid format defined in database. Defaulting to dd-mm");
                    "%0d-%0m"
                }
            },
        },
    }
    .to_string();

    let people_map = match database.get("people") {
        None => return Ok((HashMap::new(), format_pref)),
        Some(s) => match s.as_object() {
            None => return Ok((HashMap::new(), format_pref)),
            Some(o) => o,
        },
    };
    let people_map = HashMap::from_iter(
        people_map
            .iter()
            .map(|(name, person)| match person.as_object() {
                None => (name.to_owned(), Map::new()),
                Some(p) => (name.to_owned(), p.to_owned()),
            })
            .map(|(name, person)| {
                let birthday = match person.get("birthday") {
                    None => String::new(),
                    Some(b) => match b.as_str() {
                        None => "Entry corrupted in database file".to_string(),
                        Some(b) => b.to_string(),
                    },
                };
                let fields: HashMap<String, String> = match person.get("fields") {
                    None => HashMap::new(),
                    Some(f) => match f.as_object() {
                        None => HashMap::new(),
                        Some(o) => HashMap::from_iter(o.iter().map(|(k, v)| match v.as_str() {
                            None => (k.to_owned(), "Entry corrupted in database file".to_string()),
                            Some(s) => (k.to_owned(), s.to_string()),
                        })),
                    },
                };
                (name, Person { birthday, fields })
            }),
    );

    Ok((people_map, format_pref))
}

pub fn write_people(people: &HashMap<String, Person>, fmt: String) -> Result<(), String> {
    let mut savefile = File::create("birthdays.json").map_err(|e| e.to_string())?;
    let mut to_save = HashMap::new();
    _ = to_save.insert("people", people);
    let serialized = serde_json::to_string(&to_save).map_err(|e| e.to_string())?;
    let mut deserialized = match serde_json::from_str::<Value>(&serialized)
        .map_err(|e| e.to_string())
        .unwrap()
        .as_object()
    {
        None => panic!("Impossible"),
        Some(o) => o.to_owned(),
    };
    _ = deserialized.insert("format".to_string(), Value::String(fmt));
    let serialized = serde_json::to_string(&deserialized)
        .map_err(|e| e.to_string())
        .unwrap();
    write!(savefile, "{serialized}").map_err(|e| e.to_string())?;
    Ok(())
}

pub fn equal_day_and_month(date1: &NaiveDate, date2: &NaiveDate) -> bool {
    if (date1.month() == date2.month()) && (date1.day() == date2.day()) {
        return true;
    }
    false
}

pub fn parse_birthday(date: &str, fmt: &String) -> Result<NaiveDate, chrono::ParseError> {
    let fmt = &format!("{fmt}-%Y");
    match NaiveDate::parse_from_str(&format!("{date}-2001"), fmt) {
        Ok(d) => Ok(d),
        Err(_e) => match NaiveDate::parse_from_str(date, fmt) {
            Ok(d) => Ok(d),
            Err(e) => Err(e),
        },
    }
}

fn clear_terminal_ansi() {
    print!("\x1B[2J\x1B[1;1H");
    stdout().flush().unwrap();
}

#[cfg(target_os = "windows")]
pub fn clear_terminal() {
    use std::process::Command;
    match Command::new("cmd").arg("/c").arg("cls").spawn() {
        Ok(mut c) => {
            _ = c.wait().expect("Error executing clear command");
        }
        Err(_) => clear_terminal_ansi(),
    }
}

#[cfg(target_os = "linux")]
pub fn clear_terminal() {
    use std::process::Command;
    match Command::new("clear").spawn() {
        Ok(mut c) => {
            _ = c.wait().expect("Error executing clear command");
        }
        Err(_) => clear_terminal_ansi(),
    }
}

#[cfg(not(any(target_os = "windows", target_os = "linux")))]
pub fn clear_terminal() {
    clear_terminal_ansi();
}
