use chrono::{DateTime, Utc};
use crate::domain::{race::Race, discipline::Discipline, location::Location};

#[derive(Debug)]
pub struct Command {
    name: String,
    distance: f64,
    discipline: Discipline,
    date: DateTime<Utc>,
    location: Location
}

pub fn handle(cmd: Command) -> usize {
    let id = 1;
    let race = Race::new(id, cmd.name, cmd.distance, cmd.discipline, cmd.location);
    
    race.id
}
