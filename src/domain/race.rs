use super::{discipline::Discipline, location::Location};

#[derive(Debug)]
pub struct Race {
    pub id: usize,
    pub name: String,
    pub distance: f64,
    pub discipline: Discipline,
    pub location: Location,
}

impl Race {
    /// Creates a race
    pub fn new(
        id: usize,
        name: String,
        distance: f64,
        discipline: Discipline,
        location: Location,
    ) -> Self {
        Race {
            id,
            name,
            distance,
            discipline,
            location,
        }
    }
}
