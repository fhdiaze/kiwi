use crate::domain::race::Race;

pub trait Client {
    fn races(&self) -> mongodb::Collection<Race>;
}
