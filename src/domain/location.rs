use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub address: String,
    pub city: String,
    pub state: String,
    pub country: String,
}

impl Location {
    pub fn new(address: String, city: String, state: String, country: String) -> Self {
        Location {
            address,
            city,
            state,
            country,
        }
    }
}
