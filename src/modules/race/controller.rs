use super::{find, get};
use crate::domain::race::Race;

pub fn handle_get() -> Race {
    let query = get::Query::new(1);
    
    get::handle(query)
}

pub fn handle_find() -> Vec<Race> {
    let query = find::Query::new(String::from(""), String::from(""), String::from(""));

    find::handle(query)
}
