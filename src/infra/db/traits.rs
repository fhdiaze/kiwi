use crate::domain::race::Race;
use mongodb::Collection;
use std::sync::Arc;

pub type DynDbClient = Arc<dyn DbClient + Send + Sync>;

pub trait DbClient {
    fn races(&self) -> &Collection<Race>;
}
