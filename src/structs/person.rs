use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct Person {
    pub birthday: String,
    pub fields: HashMap<String, String>,
}
