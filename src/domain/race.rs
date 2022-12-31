use bson::serde_helpers::{
    deserialize_hex_string_from_object_id, serialize_hex_string_as_object_id,
};
use serde::{Deserialize, Serialize};

use super::{discipline::Discipline, location::Location};

#[derive(Debug, Serialize, Deserialize)]
pub struct Race {
    #[serde(
        rename = "_id",
        serialize_with = "serialize_hex_string_as_object_id",
        deserialize_with = "deserialize_hex_string_from_object_id"
    )]
    pub id: String,
    pub name: String,
    pub distance: f64,
    pub discipline: Discipline,
    pub location: Location,
    pub image: String,
}

impl Race {
    /// Creates a race
    pub fn new(
        id: String,
        name: String,
        distance: f64,
        discipline: Discipline,
        location: Location,
        image: String
    ) -> Self {
        Race {
            id,
            name,
            distance,
            discipline,
            location,
            image
        }
    }
}
