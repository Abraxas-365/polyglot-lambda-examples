use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Pokemon {
    pub abilities: Vec<AbilityInfo>,
    pub base_experience: i32,
    pub forms: Vec<Form>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ability {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AbilityInfo {
    pub ability: Ability,
    pub is_hidden: bool,
    pub slot: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Form {
    pub name: String,
    pub url: String,
}
