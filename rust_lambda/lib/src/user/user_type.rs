use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::errors::Error;

use super::objects_values::Name;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(skip_deserializing)]
    pub id: String,
    pub name: Name,
    #[serde(rename = "pokeball_number")]
    pub number_of_pokeballs: i32,
}
impl User {
    pub fn new(name: String, number_of_pokeballs: i32) -> Result<Self, Error> {
        let name = Name::from_str(&name).ok_or(Error::validation_error(
            "name".to_string(),
            "Name must be at least 2 characters long".to_string(),
        ))?;
        Ok(User {
            id: Uuid::new_v4().to_string(),
            name,
            number_of_pokeballs,
        })
    }
}
