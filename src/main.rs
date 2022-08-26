use crate::domain::{race::Race, discipline::Discipline};

mod modules;
mod infra;
mod domain;

fn main() {
    let r = Race::new(
        1,
        "Go Rigo Go".to_string(),
        120.5,
        Discipline::Road,
        domain::location::Location {
            address: String::from("Centro"),
            city: String::from("Villavicencio"),
            state: String::from("Meta"),
            country: String::from("Colombia"),
        },
    );
    println!("{:?}", r);
}
