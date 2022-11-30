use mongodb;

use crate::{
    domain::race::Race,
    infra::{config::DbConfig, errors::error::Result},
};

use super::traits::DbClient;

pub struct Client {
    races: mongodb::Collection<Race>,
}

impl Client {
    pub async fn new(config: &DbConfig) -> Result<Self> {
        let options = mongodb::options::ClientOptions::parse(&config.connection_string).await?;
        let client = mongodb::Client::with_options(options)?;
        let db = client.database(&config.db_name);
        let races = db.collection::<Race>(&config.races_collection);

        Ok(Self { races })
    }
}

impl DbClient for Client {
    fn races(&self) -> &mongodb::Collection<Race> {
        &self.races
    }
}
