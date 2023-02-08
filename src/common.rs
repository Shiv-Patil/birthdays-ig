use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Error, ErrorKind};
use chrono::{NaiveDate, Datelike};

pub fn read_people() -> Result<HashMap<String, String>, std::io::Error> {
    let mut file = match File::open("birthdays.json") {
        Ok(f) => f,
        Err(ref e) if e.kind() == ErrorKind::NotFound => return Err(Error::new(ErrorKind::NotFound, "The database file was not found. No birthday is stored.")),
        Err(e) => return Err(e)
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    if contents.is_empty() {
        return Ok(HashMap::new());
    }
    let people: HashMap<String, String> = match serde_json::from_str(&contents) {
        Ok(p) => p,
        Err(_e) => return Err(Error::new(ErrorKind::InvalidData, "Corrupted database file"))
    };
    Ok(people)
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
        Err(_e) => match NaiveDate::parse_from_str(&format!("{}-2001", date), "%0d-%0m-%Y") {
            Ok(d) => Ok(d),
            Err(e) => return Err(e)
        }
    }
}
