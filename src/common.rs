use crate::structs::person::Person;
use chrono::{Datelike, NaiveDate};
use serde_json::{from_str, Map, Value};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, ErrorKind, Read, Write};

pub fn read_people() -> Result<HashMap<String, Person>, std::io::Error> {
    let mut file = match File::open("birthdays.json") {
        Ok(f) => f,
        Err(ref e) if e.kind() == ErrorKind::NotFound => {
            return Err(Error::new(
                ErrorKind::NotFound,
                "The database file was not found. No birthday is stored.",
            ))
        }
        Err(e) => return Err(e),
    };
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents)?;
    if contents.is_empty() {
        return Ok(HashMap::new());
    }
    let err_corrupt = |s| Error::new(ErrorKind::InvalidData, s);
    let database: Value = match from_str(&contents) {
        Ok(p) => p,
        Err(_e) => {
            return Err(err_corrupt(
                "Corrupted database file, error trying to parse json.",
            ))
        }
    };

    let people_map = match database.get("people") {
        None => return Ok(HashMap::new()),
        Some(s) => match s.as_object() {
            None => return Ok(HashMap::new()),
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
                        Some(b) => b.to_string()
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

    Ok(people_map)
}

pub fn write_people(people: &HashMap<String, Person>) -> Result<(), String> {
    let mut savefile = File::create("birthdays.json").map_err(|e| e.to_string())?;
    let mut to_save = HashMap::new();
    _ = to_save.insert("people", people);
    let serialized = serde_json::to_string(&to_save).map_err(|e| e.to_string())?;
    write!(savefile, "{serialized}").map_err(|e| e.to_string())?;
    Ok(())
}

pub fn equal_day_and_month(date1: &NaiveDate, date2: &NaiveDate) -> bool {
    if (date1.month() == date2.month()) && (date1.day() == date2.day()) {
        return true;
    }
    false
}

pub fn parse_birthday(date: &str) -> Result<NaiveDate, chrono::ParseError> {
    match NaiveDate::parse_from_str(date, "%0d-%0m-%Y") {
        Ok(d) => Ok(d),
        Err(_e) => match NaiveDate::parse_from_str(&format!("{date}-2001"), "%0d-%0m-%Y") {
            Ok(d) => Ok(d),
            Err(e) => Err(e),
        },
    }
}
