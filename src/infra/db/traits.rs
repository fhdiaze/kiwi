use std::sync::Arc;

use crate::domain::race::Race;

pub trait DbClient {
    fn races(&self) -> &mongodb::Collection<Race>;
}

pub type DynDbClient = Arc<dyn DbClient + Send + Sync>;