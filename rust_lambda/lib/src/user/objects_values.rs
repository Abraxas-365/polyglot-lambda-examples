#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Name(String);
impl Name {
    pub fn from_str(value: &str) -> Option<Self> {
        let license_plate = Name(value.to_string());
        if license_plate.validate() {
            Some(license_plate)
        } else {
            None
        }
    }

    pub fn validate(&self) -> bool {
        if self.0.is_empty() {
            return false;
        };

        if self.0.len() < 2 {
            return false;
        }
        true
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}
