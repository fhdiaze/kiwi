use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub address: String,
    pub city: String,
    pub state: String,
    pub country: String,
}

impl Location {
    pub fn new() -> Self {
        Location {
            address: "".to_string(),
            city: "bogota".to_string(),
            state: "Bogota".to_string(),
            country: "Colombia".to_string(),
        }
    }
}
